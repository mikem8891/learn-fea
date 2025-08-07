mod utils;
mod matrix;
mod ref_matrix;
mod macros;
pub mod global_matrix;

use matrix::*;
use global_matrix::*;
use ref_matrix::*;

use std::vec;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, learn-fea!");
}


#[wasm_bindgen]
pub fn main() {
    let elasticity = plane_stress_matrix(30000.0, 0.3);
    let nodes = [
        (0.0, 0.0),
        (1.0, 0.0),
        (0.0, 1.0),
        (1.0, 1.0),
    ].map(|p| Node2D::zero_at(p));
    let known_displacements = [0, 1];
    let fixed_node = [KnownType::Displacement, KnownType::Displacement];
    let forces = [
        (2, [15.0, 0.0])
    ].map(|(i, [fx, fy])| (i, [[fx], [fy]].into()));
    let elements = [
        [0, 1, 2],
        [1, 2, 3],
    ].map(|i| T3Element::new(i));
    let mut model = Fea2DStaticModel::new(elasticity);
    model.add_nodes(&nodes);
    known_displacements.iter().for_each(|&i| *model.known_at(i) = fixed_node);
    forces.iter().for_each(|&(i, f)| *model.force_at(i) = f);
    model.add_elements(&elements);
    
}

struct Fea2DStaticModel {
    elasticity: Matrix<3,3>,
    nodes: Vec<Node2D>,
    elements: Vec<T3Element>,
    stiffness: GlobalMatrix,
}

impl Fea2DStaticModel {

    fn new(elasticity: Matrix<3,3>) -> Self {
        Fea2DStaticModel { 
            elasticity,
            nodes: vec![], 
            elements: vec![], 
            stiffness: GlobalMatrix::identity(0),
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

    pub fn force_at(&mut self, index: usize) -> &mut Matrix<2, 1> {
        &mut self.nodes[index].force
    }

    fn create_stiffness_matrix(&mut self) {
        let size = 2 * &self.nodes.len();
        let mut global_stiffness = GlobalMatrix::zeros(size, size);
        for element in &self.elements {
            let stiffnesses = element.get_stiffness_matrices(&self.nodes, self.elasticity);
            for ((i, j), stiffness_ij) in stiffnesses {
                let (i_gs, j_gs) = (2 * i, 2 * j);
                for k in 0..(stiffness_ij.rows()) {
                    for l in 0..(stiffness_ij.cols()) {
                        let (m, n) = (i_gs + k, j_gs + l);
                        global_stiffness[(m, n)] += stiffness_ij[(i, j)]
                    }
                }
            }
        }
        self.stiffness = global_stiffness;
    }
    pub fn step_guass_seidel(&mut self) {
        todo!();
        //let displacements = vec![];
        //let forces = vec![];
    }
}

#[allow(non_snake_case)]
const fn plane_stress_matrix(E: f64, nu: f64) -> Matrix<3,3> {
    let Ep = E / (1.0 - nu * nu);
    let G = E / (2.0 * (1.0 + nu));
    let values = [
        [     Ep, Ep * nu, 0.0],
        [Ep * nu,      Ep, 0.0],
        [    0.0,     0.0,   G],
    ];
    Matrix::new(values)
}

type Vector<const D: usize> = Matrix<D, 1>;

type Point2D = Vector<2>;

#[derive(Debug, Clone, Copy)]
enum KnownType {
    Force, 
    Displacement,
}

#[derive(Debug, Clone, Copy)]
struct Node2D {
    position: Point2D,
    displacement: Vector<2>, 
    force: Vector<2>,
    known: [KnownType; 2],
}

impl Node2D {
    fn zero_at((x, y): (f64, f64)) -> Self {
        let position = [[x], [y]]; 
        let position = position.into();
        let displacement = Matrix::zero();
        let force = Matrix::zero();
        let known = [KnownType::Force, KnownType::Force];
        Node2D {position, displacement, force, known}
    }
}

#[derive(Debug,Clone, Copy)]
struct T3Element {
    /** indices of the nodes from the FEA model */
    indices: [usize; 3],
}

impl T3Element {
    const fn new(indices: [usize; 3]) -> Self {
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

    const fn get_indices(&self) -> &[usize; 3] {
        &self.indices
    }

    #[allow(non_snake_case)]
    fn get_stiffness_matrices(
        &self,
        nodes: &[Node2D],
        elasticity: Matrix<3,3>
    ) -> Vec<((usize, usize), Matrix<2, 2>)> {
        let mut stiffness_matrices = vec![];
        let trial_fns = self.get_trial_functions(nodes);
        let trial_grads = trial_fns.map(|tf| tf.gradient());
        for (i, &dNi) in trial_grads.iter().enumerate() {
            for (j, &dNj) in trial_grads.iter().enumerate() {
                let [[dNi_dx], [dNi_dy]] = dNi.into();
                let [[dNj_dx], [dNj_dy]] = dNj.into();
                let diff_i: Matrix<3, 2> = [
                    [dNi_dx,    0.0],
                    [   0.0, dNi_dy],
                    [dNi_dy, dNi_dx],
                ].into();
                let diff_j: Matrix<2, 3> = [
                    [dNj_dx,    0.0, dNj_dy],
                    [   0.0, dNj_dy, dNj_dx],
                ].into();
                let index = (self.indices[i], self.indices[j]);
                let stiffness_matrix = diff_j * elasticity * diff_i;
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
    fn gradient(&self) -> Vector<2> {
        let [[x0], [y0]] = self.positions[0].into();
        let [[x1], [y1]] = self.positions[1].into();
        let [[x2], [y2]] = self.positions[2].into();
        let area = (x1 - x0) * (y2 - y0) - (x2 - x0) * (y1 - y0);
        let dadx = (y2 - y0) / area;
        let dady = (x0 - x2) / area;
        let dbdx = (y0 - y1) / area;
        let dbdy = (x1 - x0) / area;
        assert_ne!(area, 0.0, "nodes are colinear");
        let (dNda, dNdb) = match self.index {
            0 => (-1.0, -1.0),
            1 => ( 1.0,  0.0),
            2 => ( 0.0,  1.0),
            _ => unreachable!(),
        };
        [[dNda * dadx + dNdb * dbdx], [dNda * dady + dNdb * dbdy]].into()
    }
}
