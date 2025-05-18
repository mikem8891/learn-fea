mod utils;
mod matrix;
mod global_matrix;

use matrix::*;
use global_matrix::*;

use std::{cell::Cell, vec};

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
    let elasticity = plane_stress_matrix(30000.0, 10000.0, 0.3);
    let nodes: Vec<_> = [
        (0.0, 0.0),
        (1.0, 0.0),
        (0.0, 1.0),
        (1.0, 1.0),
    ].map(|p| Node2D::zero_at(p)).into();
    let element_node_indices = [
        [0, 1, 2],
        [1, 2, 3],
    ];
    let elements: Vec<_> = element_node_indices
        .map(|ni| T3Element::new(&nodes, ni)).into();
    let model = Fea2DStaticModel::new(nodes, elements);



}

struct Fea2DStaticModel<const N: usize, E: Element2D<N>> {
    nodes: Vec<Node2D>,
    elements: Vec<E>,
}

impl<const N:usize, E: Element2D<N>> Fea2DStaticModel<N, E> {
    fn new(nodes: Vec<Node2D>, elements: Vec<E>) -> Self {

        const ZERO_2X2: Matrix<2, 2> = Matrix::<2, 2>::zero();
        let size = 2 * nodes.len();
        let mut global_stiffness = vec![vec![Matrix::zero(); size]; size];
        for element in elements {
            let node_indices = &element.nodes_indices;
            let stiffness_matrices = element.get_stiffness_matrices(elasticity);
            for (i, &node_i) in node_indices.iter().enumerate() {
                for (j, &node_j) in node_indices.iter().enumerate() {
                    let stiffness_ij = stiffness_matrices[i][j];
                    global_stiffness[node_i][node_j] += &stiffness_ij;
                }
            }
        }
        let mut global_stiffness = flatten(global_stiffness);

        Fea2DStaticModel { nodes, elements }
    }
}

fn get_known_force_indices(nodes: &[Node2D]) -> Vec<usize> {
    let mut indices = vec![];
    for (i, node) in nodes.iter().enumerate() {
        let (known_x, known_y) = &node.known;
        match known_x {
            NodeKnownType::Force => indices.push(2 * i),
            NodeKnownType::Displacement => {},
        }
        match known_y {
            NodeKnownType::Force => indices.push(2 * i + 1),
            NodeKnownType::Displacement => {},
        }
    }
    indices
}

fn flatten<const R: usize, const C: usize>(
    mut block_mat: Vec<Vec<Matrix<R, C>>>
) -> Vec<Vec<f64>> {
    let mut global_mat = vec![vec![]; R];
    for block_i  in 0..block_mat.len() {
        let block_row = &mut block_mat[block_i];
        for block_j in 0..block_row.len() {
            let matrix = block_row[block_j];
            for mat_i in 0..R {
                for mat_j in 0..C {
                    global_mat[R * block_i + mat_i]
                        .push(matrix[(mat_i, mat_j)]);
                }
            }
        }
        *block_row = vec![];
    }
    global_mat
}

#[allow(non_snake_case)]
const fn plane_stress_matrix(E: f64, G: f64, nu: f64) -> Matrix<3,3> {
    let Ep = E / (1.0 - nu * nu);
    let values = [
        [     Ep, Ep * nu, 0.0],
        [Ep * nu,      Ep, 0.0],
        [    0.0,     0.0,   G],
    ];
    Matrix::new(values)
}



#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct GlobalMatrix {
    rows: usize,
    cols: usize,
    values: Box<[f64]>,
}


type Vector<const D: usize> = Matrix<D, 1>;

type Point2D = Vector<2>;

enum NodeKnownType {
    Force,
    Displacement,
}

struct Node2D {
    position: Point2D,
    displacement: Cell<Vector<2>>, 
    force: Cell<Vector<2>>,
    known: (NodeKnownType, NodeKnownType),
}

impl Node2D {
    fn zero_at(position: (f64, f64)) -> Self {
        let position = position.into();
        let displacement = Cell::new(Matrix::zero());
        let force = Cell::new(Matrix::zero());
        type Known = NodeKnownType;
        let known = (Known::Force, Known::Force);
        Node2D {position, displacement, force, known }
    }
}


impl From<(f64, f64)> for Matrix<2, 1> {
    fn from((x, y): (f64, f64)) -> Self {
        let values = [[x],[y]];
        Matrix::new(values)
    }
}

trait Element2D<const N: usize> {
    fn get_stiffness_matrices(
        &self, 
        elasticity: Matrix<3, 3>
    ) -> [[Matrix<2, 2>; N]; N];
}

struct T3Element<'a> {
    nodes: &'a [Node2D],
    nodes_indices: [usize; 3],
}

impl<'a> T3Element<'a> {
    const fn new(nodes: &'a [Node2D], nodes_indices: [usize; 3]) -> Self {
        T3Element { nodes, nodes_indices }
    }

    fn get_trial_functions(&'a self) -> [T3TrailFunction<'a>; 3] {
        [
            T3TrailFunction { element: self, index: 0 },
            T3TrailFunction { element: self, index: 1 },
            T3TrailFunction { element: self, index: 2 },
        ]
    }

    const fn get_node(&self, i: usize) -> &Node2D {
        &self.nodes[self.nodes_indices[i]]
    }
    const fn get_position(&self, i: usize) -> &Point2D {
        &self.get_node(i).position
    }

    fn area(&self) -> f64 {
        let n_0 = self.get_position(0);
        let n_1 = self.get_position(1);
        let n_2 = self.get_position(2);
        let n_10 = n_1 - n_0;
        let n_20 = n_2 - n_0;
        n_10.x() * n_20.y() - n_10.y() * n_20.x()
    }
}

impl Element2D<3> for T3Element<'_> {
    #[allow(non_snake_case)]
    fn get_stiffness_matrices(
        &self, 
        elasticity: Matrix<3,3>
    ) -> [[Matrix<2, 2>; 3]; 3] {
        let mut stiffness_matrices = [[Matrix::zero(); 3]; 3];
        let trial_fns = self.get_trial_functions();
        let trial_grads = trial_fns.map(|tf| tf.gradient().into());
        for (i, &(dNi_dx, dNi_dy)) in trial_grads.iter().enumerate() {
            for (j, &(dNj_dx, dNj_dy)) in trial_grads.iter().enumerate() {
                let diff_i: Matrix<3, 2> = [
                    [dNi_dx,    0.0],
                    [   0.0, dNi_dy],
                    [dNi_dy, dNi_dx],
                ].into();
                let diff_j: Matrix<2, 3> = [
                    [dNj_dx,    0.0, dNj_dy],
                    [   0.0, dNj_dy, dNj_dx],
                ].into();
                stiffness_matrices[i][j] = diff_j * elasticity * diff_i;
            }
        }
        stiffness_matrices
    }
}

struct T3TrailFunction<'a> {
    element: &'a T3Element<'a>,
    index: u8, 
}

impl<'a> T3TrailFunction<'a> {

    #[allow(non_snake_case)]
    fn gradient(&self) -> Vector<2> {
        let (x0, y0) = self.element.get_position(0).into();
        let (x1, y1) = self.element.get_position(1).into();
        let (x2, y2) = self.element.get_position(2).into();
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
