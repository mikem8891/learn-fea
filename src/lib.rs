mod utils;
mod matrix;

use matrix::*;

use std::{cell::Cell, ops::{Add, AddAssign, Index, IndexMut, Mul, Sub}};

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
    let nodes = [
        (0.0, 0.0),
        (1.0, 0.0),
        (0.0, 1.0),
        (1.0, 1.0),
    ].map(|p| Node2D::zero_at(p));
    let element_node_indices = [
        [0, 1, 2],
        [1, 2, 3],
    ];
    let elements = element_node_indices
        .map(|ni| T3Element::new(&nodes, ni));
    const ZERO_2X2: Matrix<2, 2> = Matrix::<2, 2>::zero();
    let size = 2 * nodes.len();
    let mut global_stiffness = vec![vec![0.0; size]; size];
    for element in elements {
        let node_indices = &element.nodes_indices;
        let stiffness_matrices = element.get_stiffness_matrices(elasticity);
        for (i, &node_i) in node_indices.iter().enumerate() {
            for (j, &node_j) in node_indices.iter().enumerate() {
                let stiffness_ij = stiffness_matrices[i][j];
                global_stiffness[node_i + 0][node_j + 0] += stiffness_ij[(0, 0)];
                global_stiffness[node_i + 1][node_j + 0] += stiffness_ij[(1, 0)];
                global_stiffness[node_i + 0][node_j + 1] += stiffness_ij[(0, 1)];
                global_stiffness[node_i + 1][node_j + 1] += stiffness_ij[(1, 1)];
            }
        }
    }

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

trait Element2D {
    fn contains(point: Point2D) -> bool;
    fn value_at(point: Point2D) -> Vector<2>;

}

struct T3Element<'a> {
    nodes: &'a [Node2D],
    nodes_indices: [usize; 3],
}

impl<'a> T3Element<'a> {
    const fn new(nodes: &'a [Node2D], nodes_indices: [usize; 3]) -> Self {
        T3Element { nodes, nodes_indices }
    }

    #[allow(non_snake_case)]
    fn get_stiffness(&self, i: usize, j: usize) -> Matrix<2, 2> {
        let trial = self.get_trial_functions();
        let (dni_dx, dni_dy) = trial[i].gradient().into();
        let diff_i: Matrix<3, 2> = [
            [dni_dx,    0.0],
            [   0.0, dni_dy],
            [dni_dy, dni_dx],
        ].into();
        let (dnj_dx, dnj_dy) = trial[j].gradient().into();
        let diff_j: Matrix<2, 3> = [
            [dnj_dx,    0.0, dnj_dy],
            [   0.0, dnj_dy, dnj_dx],
        ].into();
        let E = plane_stress_matrix(30000.0, 10000.0, 0.3);
        diff_j * E * diff_i
    }

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

#[wasm_bindgen]
impl GlobalMatrix {

    fn new(matrix: &[&[f64]]) -> Self {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut new_mat = Self::zeros(rows, cols);
        for row in 0..rows {
            for col in 0..cols {
                new_mat[(row, col)] = matrix[row][col];
            }
        }
        new_mat
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        let values = vec![0.0; rows * cols].into_boxed_slice();
        GlobalMatrix {rows, cols, values}
    }

    pub fn identity(size: usize) -> Self {
        let mut matrix = Self::zeros(size, size);
        for diag in 0..size {
            matrix[(diag, diag)] = 1.0;
        }
        matrix
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self[(row, col)]
    }

    pub fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }
    pub fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl Index<(usize, usize)> for GlobalMatrix {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        let index = row * self.cols + col; 
        &self.values[index]
    }
}

impl IndexMut<(usize, usize)> for GlobalMatrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        let index = row * self.cols + col; 
        &mut self.values[index]
    }
}

impl Add for &GlobalMatrix {
    type Output = GlobalMatrix;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = GlobalMatrix::zeros(self.rows, self.cols);
        for row in 0..self.rows {
            for col in 0..self.cols {
                sum[(row, col)] = self[(row, col)] + rhs[(row, col)];
            }
        }
        sum
    }
}

impl Mul for &GlobalMatrix {
    type Output = GlobalMatrix;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols, rhs.rows);

        let mut prod = GlobalMatrix::zeros(self.rows, rhs.cols);
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                for k in 0..self.cols {
                    prod[(i, j)] += self[(i, k)] * rhs[(k, j)];
                }
            }
        }
        prod
    }
}

