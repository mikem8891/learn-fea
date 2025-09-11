
pub mod test;

use std::{cell::RefCell, fmt, rc::Rc, vec};
use crate::math::{stack::Matrix, *};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Lin2DStaticModel {
    elasticity: stack::Matrix<3,3>,
    nodes: Rc<RefCell<Vec<Node2D>>>,
    elements: Vec<T3Element>,
    stiffness: Option<heap::Matrix>,
}

impl Lin2DStaticModel {

    pub fn new(elasticity: stack::Matrix<3,3>) -> Self {
        Lin2DStaticModel { 
            elasticity,
            nodes: Rc::new(RefCell::new(vec![])), 
            elements: vec![], 
            stiffness: None,
        }
    }

    pub fn add_nodes(&mut self, nodes: &[Node2D]) {
        self.nodes.borrow_mut().extend_from_slice(nodes);
    }

    pub fn add_elements(&mut self, elements: &[[usize;3]]) {
        for &indices in elements {
            self.elements.push(T3Element::new(self.get_nodes(), indices));
        }
        self.create_stiffness_matrix();
    }

    pub fn get_nodes(&self) -> Rc<RefCell<Vec<Node2D>>> {
        Rc::clone(&self.nodes)
    }

    fn create_stiffness_matrix(&mut self) {
        let size = 2 * self.nodes.borrow().len();
        let mut global_stiffness = heap::Matrix::zeros(size, size);
        for element in &self.elements {
            let stiffnesses = element.get_stiffness_matrices(self.elasticity);
            for ((i, j), stiffness_ij) in stiffnesses {
                let (i_gs, j_gs) = (2 * j, 2 * i);
                for k in 0..(stiffness_ij.rows()) {
                    for l in 0..(stiffness_ij.cols()) {
                        let (m, n) = (i_gs + k, j_gs + l);
                        global_stiffness[(m, n)] += stiffness_ij[(k, l)]
                    }
                }
            }
        }
        self.stiffness = Some(global_stiffness);
    }

    pub fn step_guass_seidel(&self, steps: usize) {
        let stiffness = self.stiffness.as_ref().unwrap();
        let mut nodes = self.nodes.borrow_mut();
        let len = nodes.len();
        let mut u = vec![0.0; 2 * len];
        let mut f = vec![0.0; 2 * len];
        let mut known_u = vec![];
        let mut known_f = vec![];
        for i in 0..len {
            let node = nodes[i];
            for j in 0..2 {
                let idx = 2 * i + j;
                u[idx] = node.displacement[j];
                f[idx] = node.force[j];
                match node.known[j] {
                    KnownType::Displacement =>
                        known_u.push(idx),
                    KnownType::Force =>
                        known_f.push(idx),
                }
            }
        }
        let mut u = heap::Vector::new(u.into_boxed_slice());
        let mut f = heap::Vector::new(f.into_boxed_slice());
        for _step in 0..steps {
            for &i in &known_f {
                let k_i = &stiffness[i];
                u[i] = (f[i] - k_i * &u) / k_i[i] + u[i];
            }
        }
        for i in known_u {
            let k_i = &stiffness[i];
            f[i] = k_i * &u;
        }
        for i in 0..len {
            let node = &mut nodes[i];
            for j in 0..2 {
                node.displacement[j] = u[2 * i + j];
                node.force[j]        = f[2 * i + j];
            }
        }
    }
}

#[wasm_bindgen]
impl Lin2DStaticModel {
    pub fn nodes_len(&self) -> usize {
        self.nodes.borrow().len()
    }
    pub fn add_node(&mut self) {
        self.nodes.borrow_mut().push(Node2D::zero_at((0.0, 0.0)));
    }
    pub fn delete_node(&mut self, index: usize) {
        let mut nodes = self.nodes.borrow_mut();
        nodes.swap_remove(index);
        let (i_new, i_old) = (index, nodes.len());
        for element in &mut self.elements {
            for index in &mut element.indices {
                if i_old == *index {
                    *index = i_new;
                } else if i_new == *index {
                    *index = i_old;
                }
            }
        }
    }
    pub fn get_node(&self, index: usize) -> Node2D { 
        let nodes = self.nodes.borrow(); 
        nodes[index]
    }
    pub fn set_node(&mut self, index: usize, node: &Node2D) {
        let mut nodes = self.nodes.borrow_mut(); 
        nodes[index] = *node;
    }
    pub fn elements_len(&self) -> usize {
        self.elements.len()
    }
    pub fn add_elem(&mut self) {
        let len = self.nodes_len();
        self.add_elements(&[[len - 3, len - 2, len - 1]]);
        self.stiffness = None;
    }
    pub fn delete_element(&mut self, index: usize) {
        self.elements.swap_remove(index);
        self.stiffness = None;
    }
    pub fn get_element_indices(&self, index: usize) -> Box<[usize]> {
        self.elements[index].indices.into()
    }
    pub fn set_element_indices(
        &mut self, element_index: usize, new_indices: &[usize]
    ) {
        self.elements[element_index].indices.copy_from_slice(new_indices);
    }
    pub fn step(&mut self) -> Result<(), JsError>{
        self.check().map_err(|e| JsError::from(e))?;
        self.create_stiffness_matrix();
        self.step_guass_seidel(1);
        Ok(())
    }
    pub fn check(&self) -> Result<(), JsError> {
        let nodes = self.nodes.borrow();
        let mut errors = vec![];
        let mut node_in_element = vec![false; nodes.len()];
        for (element_index, element) in self.elements.iter().enumerate() {
            let mut missing_nodes = false;
            for &node_index in &element.indices {
                if node_index >= nodes.len() {
                    // check no nodes are missing from elements
                    errors.push(FeaErrorType::MissingNode {node_index, element_index});
                    missing_nodes = true;
                } else {
                    // check all known force nodes are part of an element
                    node_in_element[node_index] = true;
                }
            }
            // Check all elements have size
            if !missing_nodes && element.area() <= 0.0 {
                errors.push(FeaErrorType::DegenerateElement {element_index});
            }
        }
        for (node_index, &in_element) in node_in_element.iter().enumerate() {
            if !in_element {
                errors.push(FeaErrorType::NodeWithoutElement {node_index});
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(FeaError{errors}.into())
        }
    }
}

#[allow(non_snake_case)]
pub fn plane_stress_matrix(E: f64, nu: f64, G: f64) -> stack::Matrix<3,3> {
    let Ep = E / (1.0 - nu * nu);
    let values = [
        [     Ep, Ep * nu, 0.0],
        [Ep * nu,      Ep, 0.0],
        [    0.0,     0.0,   G],
    ];
    stack::Matrix::new(values)
}

#[derive(Debug, Clone, Copy)]
enum FeaErrorType {
    MissingNode {
        node_index: usize,
        element_index: usize,
    },
    NodeWithoutElement {
        node_index: usize
    },
    DegenerateElement {
        element_index: usize
    },
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct FeaError {
    errors: Vec<FeaErrorType>
}

impl fmt::Display for FeaErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeaErrorType::MissingNode {node_index, element_index} => 
                write!(f, "node (index: {node_index}) is missing from element (index: {element_index}) and should be replaced"),
            FeaErrorType::NodeWithoutElement { node_index } => 
                write!(f, "node (index: {node_index}) is not part of an element and should be deleted"),
            FeaErrorType::DegenerateElement { element_index } => 
                write!(f, "element (index: {element_index}) has no size and should be deleted"),
        }
    }
}

impl std::error::Error for FeaErrorType {}

impl fmt::Display for FeaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for error in &self.errors {
            writeln!(f, "{error}")?
        }
        Ok(())
    }
}

impl std::error::Error for FeaError {}

type Point2D = stack::Vector<2>;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum KnownType {
    Force, 
    Displacement,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Node2D {
    position: Point2D,
    displacement: stack::Vector<2>, 
    force: stack::Vector<2>,
    known: [KnownType; 2],
}

impl Node2D {
    pub fn zero_at((x, y): (f64, f64)) -> Self {
        use stack::Vector;
        let position = [x, y]; 
        let position = Vector::new(position);
        let displacement = Vector::zeros();
        let force = Vector::zeros();
        let known = [KnownType::Force, KnownType::Force];
        Node2D {position, displacement, force, known}
    }
}

#[wasm_bindgen]
impl Node2D {
    #[wasm_bindgen(getter = posX)]
    pub fn get_pos_x(&self) -> f64 {
        self.position.x()
    }
    #[wasm_bindgen(getter = posY)]
    pub fn get_pos_y(&self) -> f64 {
        self.position.y()
    }
    #[wasm_bindgen(setter = posX)]
    pub fn set_pos_x(&mut self, value: f64) {
        self.position[0] = value;
    }
    #[wasm_bindgen(setter = posY)]
    pub fn set_pos_y(&mut self, value: f64) {
        self.position[1] = value;
    }
    #[wasm_bindgen(getter = dispX)]
    pub fn get_disp_x(&self) -> f64 {
        self.displacement.x()
    }
    #[wasm_bindgen(getter = dispY)]
    pub fn get_disp_y(&self) -> f64 {
        self.displacement.y()
    }
    #[wasm_bindgen(setter = dispX)]
    pub fn set_disp_x(&mut self, value: f64) {
        self.displacement[0] = value;
    }
    #[wasm_bindgen(setter = dispY)]
    pub fn set_disp_y(&mut self, value: f64) {
        self.displacement[1] = value;
    }
    #[wasm_bindgen(getter = forceX)]
    pub fn get_force_x(&self) -> f64 {
        self.force.x()
    }
    #[wasm_bindgen(getter = forceY)]
    pub fn get_force_y(&self) -> f64 {
        self.force.y()
    }
    #[wasm_bindgen(setter = forceX)]
    pub fn set_force_x(&mut self, value: f64) {
        self.force[0] = value;
    }
    #[wasm_bindgen(setter = forceY)]
    pub fn set_force_y(&mut self, value: f64) {
        self.force[1] = value;
    }
    #[wasm_bindgen(getter = knownX)]
    pub fn get_known_x(&self) -> KnownType {
        self.known[0]
    }
    #[wasm_bindgen(getter = knownY)]
    pub fn get_known_y(&self) -> KnownType {
        self.known[1]
    }
    #[wasm_bindgen(setter = knownX)]
    pub fn set_known_x(&mut self, known: KnownType) {
        self.known[0] = known;
    }
    #[wasm_bindgen(setter = knownY)]
    pub fn set_known_y(&mut self, known: KnownType) {
        self.known[1] = known;
    }
}


#[wasm_bindgen]
#[derive(Debug)]
pub struct T3Element {
    nodes: Rc<RefCell<Vec<Node2D>>>,
    indices: [usize; 3],
}

impl T3Element {
    pub const fn new(nodes: Rc<RefCell<Vec<Node2D>>>, indices: [usize; 3]) -> Self {
        T3Element { nodes, indices }
    }
    fn pos(&self) -> [Point2D; 3] {
        let nodes = self.nodes.borrow();
        self.indices.map(|i| nodes[i].position)
    }
    fn get_pos(&self) -> [Option<Point2D>; 3] {
        let nodes = self.nodes.borrow();
        self.indices.map(|i| nodes.get(i).map(|n| n.position))
    }

    fn get_trial_functions(&self) -> [T3TrailFunction; 3] {
        let pos = self.pos();
        [
            T3TrailFunction {positions: pos},
            T3TrailFunction {positions: [pos[1], pos[2], pos[0]]},
            T3TrailFunction {positions: [pos[2], pos[0], pos[1]]},
        ]
    }
    fn get_strain_ops(&self) -> [Matrix<3, 2>; 3] {
        let trial_fns = self.get_trial_functions();
        let trial_grads = trial_fns.map(|tf| tf.gradient());
        trial_grads.map(|grad_n|
            [
                [grad_n.x(),        0.0],
                [       0.0, grad_n.y()],
                [grad_n.y(), grad_n.x()],
            ].into())
    }

    fn get_stress_and_strain_ops(&self) -> 
    ([Matrix<2, 3>;3], [Matrix<3, 2>;3]) {
        let strain_ops = self.get_strain_ops();
        let stress_ops = strain_ops.map(|s| s.transpose());
        (stress_ops, strain_ops)
    }

    fn get_strain(&self) -> stack::Vector<3> {
        let mut strain = stack::Vector::zeros();
        let nodes = self.nodes.borrow();
        let strain_ops = self.get_strain_ops();
        let displacements = self.indices.map(|i| nodes[i].displacement);
        for (n, u) in std::iter::zip(strain_ops, displacements) {
            strain += n * u;
        }
        strain
    }

    fn get_stress(&self, elasticity: stack::Matrix<3,3>) -> stack::Vector<3> {
        elasticity * self.get_strain()
    }

    fn area(&self) -> f64 {
        let pos = self.pos();
        let d1 = pos[1] - pos[0];
        let d2 = pos[2] - pos[0];
        0.5 * d1.cross(d2).abs()
    }

    #[allow(non_snake_case)]
    fn get_stiffness_matrices(
        &self,
        elasticity: stack::Matrix<3,3>
    ) -> Vec<((usize, usize), stack::Matrix<2, 2>)> {
        let area = self.area();
        let mut stiffness_matrices = vec![];
        let (stress_ops, strain_ops) = self.get_stress_and_strain_ops();
        for (i, &strain_op) in strain_ops.iter().enumerate() {
            for (j, &stress_op) in stress_ops.iter().enumerate() {
                let index = (self.indices[i], self.indices[j]);
                let stiffness_mat = stress_op * elasticity * strain_op * area;
                stiffness_matrices.push((index, stiffness_mat));
            }
        }
        stiffness_matrices
    }
}

struct T3TrailFunction {
    positions: [Point2D; 3],
}

impl T3TrailFunction {

    #[allow(non_snake_case)]
    fn gradient(&self) -> stack::Vector<2> {
        let pos = &self.positions;
        let [[da_dx, da_dy], [db_dx, db_dy]] = stack::Matrix::new([
            [pos[1].x() - pos[0].x(), pos[2].x() - pos[0].x()],
            [pos[1].y() - pos[0].y(), pos[2].y() - pos[0].y()],
        ]).inv().into();
        let (dN_da, dN_db) = (-1.0, -1.0);
        [dN_da * da_dx + dN_db * db_dx, dN_da * da_dy + dN_db * db_dy].into()
    }
}
