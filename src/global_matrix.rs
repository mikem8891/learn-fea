use std::ops::{Add, Index, IndexMut, Mul};

use crate::matrix::Matrix;

#[derive(Debug, Clone)]
pub struct GlobalMatrix {
    values: Box<[Box<[f64]>]>,
}

impl GlobalMatrix {

    pub fn new<T>(matrix: &[T]) -> Self 
    where for<'a> &'a T: IntoIterator<Item = f64>{
        let mut values = vec![];
        for row in matrix {
            let mut global_row = vec![];
            for value in row {
                global_row.push(value);
            }
            values.push(global_row.into_boxed_slice());
        }
        let values = values.into_boxed_slice();
        GlobalMatrix { values }
    }

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

    fn num_of_rows(&self) -> usize {
        self.values.len()
    }

    fn num_of_cols(&self) -> usize {
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

impl Add for &GlobalMatrix {
    type Output = GlobalMatrix;

    fn add(self, rhs: Self) -> Self::Output {
        let (num_of_rows, num_of_cols) = self.size();
        let mut sum = GlobalMatrix::zeros(num_of_rows, num_of_cols);
        for row in 0..num_of_rows {
            for col in 0..num_of_cols {
                sum[(row, col)] = self[(row, col)] + rhs[(row, col)];
            }
        }
        sum
    }
}

impl Mul for &GlobalMatrix {
    type Output = GlobalMatrix;

    #[allow(non_snake_case)]
    fn mul(self, rhs: Self) -> Self::Output {

        let (I, J) = self.size();
        let K = rhs.num_of_cols();
        assert_eq!(self.num_of_cols(), rhs.num_of_rows());

        let mut prod_matrix = GlobalMatrix::zeros(I, K);
        for i in 0..I {
            for k in 0..K {
                let prod = (0..J).map(|j| self[(i, j)] * rhs[(j, k)]).sum();
                prod_matrix[(i, k)] = prod;
            }
        }
        prod_matrix
    }
}
