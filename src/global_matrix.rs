use std::ops::{Index, IndexMut};

use crate::matrix::Matrix;

#[derive(Debug, Clone)]
pub struct GlobalMatrix {
    values: Box<[Box<[f64]>]>,
}

impl GlobalMatrix {

    pub fn zeros(rows: usize, cols: usize) -> Self {
        let row = vec![0.0; cols].into_boxed_slice();
        let values = vec![row; rows].into_boxed_slice();
        GlobalMatrix { values }
    }

    pub fn identity(size: usize) -> Self {
        let mut matrix = Self::zeros(size, size);
        for diag in 0..size {
            matrix[(diag, diag)] = 1.0;
        }
        matrix
    }

    fn rows(&self) -> usize {
        self.values.len()
    }

    fn cols(&self) -> usize {
        self.values[0].len()
    }

    fn size(&self) -> (usize, usize) {
        (self.values.len(), self.values[0].len())
    }

    pub fn add_assign_with<const R: usize, const C: usize> (
        &mut self, 
        matrix: Matrix<R, C>,
        dest: (usize, usize)
    ) {
        for i in 0..R {
            for j in 0..C {
                self[dest] += matrix[(i, j)];
            }
        }
    }
}

impl Index<(usize, usize)> for GlobalMatrix {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.values[row][col]
    }
}

impl IndexMut<(usize, usize)> for GlobalMatrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.values[row][col]
    }
}
