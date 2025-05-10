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
    let nodes = [
        (0.0, 0.0),
        (1.0, 0.0),
        (0.0, 1.0),
        (1.0, 1.0),
    ].map(|p| Node2D::new(p));
    let element_node_indices = [
        [0, 1, 2],
        [1, 2, 3],
    ];
    let elements = element_node_indices
        .map(|ni| T3Element::new(&nodes, ni));
    const ZERO_2X2: Matrix<2, 2> = Matrix::<2, 2>::zero();
    let mut global_stiffness = vec![vec![ZERO_2X2; nodes.len()]; nodes.len()];
    for element in elements {
        let node_indices = &element.nodes_indices;
        for (i, &node_i) in node_indices.iter().enumerate() {
            for (j, &node_j) in node_indices.iter().enumerate() {
                let stiffness_ij = element.get_stiffness(i, j);
                global_stiffness[node_i][node_j] += &stiffness_ij;
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

struct Node2D {
    position: Point2D,
    value:    Cell<Vector<2>>,
}

impl Node2D {
    fn new(position: (f64, f64)) -> Self {
        let position = position.into();
        let value = Cell::new(Matrix::zero());
        Node2D {position, value}
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

    fn get_stiffness(&self, i: usize, j: usize) -> Matrix<2, 2> {
        let diff_i = 0;

        todo!()
    }

    fn area(&self) -> f64 {
        let n_0 = &self.nodes[self.nodes_indices[0]].position;
        let n_1 = &self.nodes[self.nodes_indices[1]].position;
        let n_2 = &self.nodes[self.nodes_indices[2]].position;
        let n_10 = n_1 - n_0;
        let n_20 = n_2 - n_0;
        n_10.x() * n_20.y() - n_10.y() * n_20.x()
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

