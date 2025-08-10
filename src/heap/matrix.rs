use std::ops::{Index, IndexMut};

use crate::stack;
use crate::heap::Vector;

#[derive(Debug, Clone)]
pub struct Matrix {
    rows: Box<[Vector]>,
}

impl Matrix {

    pub fn zeros(rows: usize, cols: usize) -> Self {
        let row = Vector::zeros(cols);
        let rows = vec![row; rows].into_boxed_slice();
        Matrix { rows }
    }

    pub fn identity(size: usize) -> Self {
        let mut matrix = Self::zeros(size, size);
        for diag in 0..size {
            matrix[(diag, diag)] = 1.0;
        }
        matrix
    }

    fn rows(&self) -> usize {
        self.rows.len()
    }

    fn cols(&self) -> usize {
        self.rows[0].len()
    }

    fn size(&self) -> (usize, usize) {
        (self.rows.len(), self.rows[0].len())
    }

    pub fn add_assign_with<const R: usize, const C: usize> (
        &mut self, 
        matrix: stack::Matrix<R, C>,
        dest: (usize, usize)
    ) {
        for i in 0..R {
            for j in 0..C {
                self[dest] += matrix[(i, j)];
            }
        }
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self[row][col]
    }
}

impl Index<usize> for Matrix {
    type Output = Vector;

    fn index(&self, row: usize) -> &Self::Output {
        &self.rows[row]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self[row][col]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.rows[row]
    }
}
