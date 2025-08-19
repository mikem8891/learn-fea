
use std::vec;
use crate::math::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Lin2DStaticModel {
    elasticity: stack::Matrix<3,3>,
    nodes: Vec<Node2D>,
    elements: Vec<T3Element>,
    stiffness: heap::Matrix,
}

impl Lin2DStaticModel {

    pub fn new(elasticity: stack::Matrix<3,3>) -> Self {
        Lin2DStaticModel { 
            elasticity,
            nodes: vec![], 
            elements: vec![], 
            stiffness: heap::Matrix::identity(0),
        }
    }

    pub fn add_nodes(&mut self, nodes: &[Node2D]) {
        self.nodes.extend_from_slice(nodes);
    }

    pub fn add_elements(&mut self, elements: &[T3Element]) {
        self.elements.extend_from_slice(elements);
        self.create_stiffness_matrix();
    }

    pub fn known_at(&mut self, index: usize) -> &mut [KnownType; 2] {
        &mut self.nodes[index].known
    }

    pub fn displacement_at(&mut self, index: usize) -> &mut stack::Vector<2> {
        &mut self.nodes[index].displacement
    }

    pub fn force_at(&mut self, index: usize) -> &mut stack::Vector<2> {
        &mut self.nodes[index].force
    }

    fn create_stiffness_matrix(&mut self) {
        let size = 2 * &self.nodes.len();
        let mut global_stiffness = heap::Matrix::zeros(size, size);
        for element in &self.elements {
            let stiffnesses = element.get_stiffness_matrices(&self.nodes, self.elasticity);
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
        log!("{:?}", &global_stiffness);
        self.stiffness = global_stiffness;
    }

    pub fn step_guass_seidel(&mut self, steps: usize) {
        let len = self.nodes.len();
        let mut u = vec![0.0; 2 * len];
        let mut f = vec![0.0; 2 * len];
        let mut known_u = vec![];
        let mut known_f = vec![];
        for i in 0..len {
            let node = self.nodes[i];
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
                let k_i = &self.stiffness[i];
                u[i] = (f[i] - k_i * &u) / k_i[i] + u[i];
            }
        }
        for i in known_u {
            let k_i = &self.stiffness[i];
            f[i] = k_i * &u;
        }
        for i in 0..len {
            let node = &mut self.nodes[i];
            for j in 0..2 {
                node.displacement[j] = u[2 * i + j];
                node.force[j]        = f[2 * i + j];
            }
        }
    }
}

#[allow(non_snake_case)]
pub fn plane_stress_matrix(Ep: f64, nu: f64, G: f64) -> stack::Matrix<3,3> {
    let values = [
        [     Ep, Ep * nu, 0.0],
        [Ep * nu,      Ep, 0.0],
        [    0.0,     0.0,   G],
    ];
    stack::Matrix::new(values)
}

type Point2D = stack::Vector<2>;

#[derive(Debug, Clone, Copy)]
pub enum KnownType {
    Force, 
    Displacement,
}

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

#[derive(Debug,Clone, Copy)]
pub struct T3Element {
    /** indices of the nodes from the FEA model */
    indices: [usize; 3],
}

impl T3Element {
    pub const fn new(indices: [usize; 3]) -> Self {
        T3Element { indices }
    }

    fn get_trial_functions(&self, nodes: &[Node2D]) -> [T3TrailFunction; 3] {
        let positions = [0, 1, 2].map(|i| nodes[self.indices[i]].position);
        [
            T3TrailFunction { positions, index: 0 },
            T3TrailFunction { positions, index: 1 },
            T3TrailFunction { positions, index: 2 },
        ]
    }

//    const fn get_indices(&self) -> &[usize; 3] {
//        &self.indices
//    }

    fn area(&self, nodes: &[Node2D]) -> f64 {
        use std::array;
        let pos: [_; 3] = array::from_fn(|i| nodes[self.indices[i]].position);
        let d1 = pos[1] - pos[0];
        let d2 = pos[2] - pos[0];
        d1.cross(d2) / 2.0
    }

    #[allow(non_snake_case)]
    fn get_stiffness_matrices(
        &self,
        nodes: &[Node2D],
        elasticity: stack::Matrix<3,3>
    ) -> Vec<((usize, usize), stack::Matrix<2, 2>)> {
        let mut stiffness_matrices = vec![];
        let trial_fns = self.get_trial_functions(nodes);
        let trial_grads = trial_fns.map(|tf| tf.gradient());
        for (i, &dNi) in trial_grads.iter().enumerate() {
            for (j, &dNj) in trial_grads.iter().enumerate() {
                let [dNi_dx, dNi_dy] = dNi.into();
                let [dNj_dx, dNj_dy] = dNj.into();
                let diff_i: stack::Matrix<3, 2> = [
                    [dNi_dx,    0.0],
                    [   0.0, dNi_dy],
                    [dNi_dy, dNi_dx],
                ].into();
                let diff_j: stack::Matrix<2, 3> = [
                    [dNj_dx,    0.0, dNj_dy],
                    [   0.0, dNj_dy, dNj_dx],
                ].into();
                let index = (self.indices[i], self.indices[j]);
                let area = self.area(nodes).abs();
                let stiffness_matrix = diff_j * elasticity * diff_i * area;
                stiffness_matrices.push((index, stiffness_matrix));
            }
        }
        stiffness_matrices
    }
}

struct T3TrailFunction {
    positions: [Point2D; 3],
    /** index (0, 1, 2) of the trial function */
    index: u8, 
}

impl T3TrailFunction {

    #[allow(non_snake_case)]
    fn gradient(&self) -> stack::Vector<2> {
        use stack::Vector;
        use std::array;
        let pos: [_; 3] = array::from_fn(|i| self.positions[i]);
        let [delta_1, delta_2] = array::from_fn(|i| pos[i + 1] - pos[0]);
        let cross = delta_1.cross(delta_2);
        assert_ne!(cross, 0.0, "nodes are colinear");
        let grad_alpha = Vector::new([ delta_2.y(), -delta_2.x()]) / cross;
        let grad_beta  = Vector::new([-delta_1.y(),  delta_1.x()]) / cross;
        let (dNda, dNdb) = match self.index {
            0 => (-1.0, -1.0),
            1 => ( 1.0,  0.0),
            2 => ( 0.0,  1.0),
            _ => unreachable!(),
        };
        dNda * grad_alpha + dNdb * grad_beta
    }
}
