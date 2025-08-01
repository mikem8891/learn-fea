
#[cfg(test)]
mod test;

use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const R: usize, const C: usize> {
    values: [[f64; C]; R]
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub const fn zero() -> Self {
        let values = [[0.0; C]; R];
        Matrix {values}
    }
    pub const fn new(values: [[f64; C]; R]) -> Self {
        Matrix {values}
    }
}

impl<const R: usize, const C: usize> From<[[f64; C]; R]> for Matrix<R, C> {
    fn from(values: [[f64; C]; R]) -> Self {
        Matrix {values}
    }
}

impl<const R: usize, const C: usize> From<Matrix<R, C>> for [[f64; C]; R] {
    fn from(matrix: Matrix<R, C>) -> Self {
        matrix.values
    }
}


impl<const R: usize, const C: usize> AddAssign<Matrix<R, C>> for Matrix<R, C> {
    fn add_assign(&mut self, rhs: Matrix<R, C>) {
        for row in 0..R {
            for col in 0..C {
                self[(row, col)] += rhs[(row, col)];
            }
        }
    }
}

impl<const R: usize, const C: usize> Add for Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const R: usize, const C: usize> SubAssign<Matrix<R, C>> for Matrix<R, C> {
    fn sub_assign(&mut self, rhs: Matrix<R, C>) {
        for row in 0..R {
            for col in 0..C {
                self[(row, col)] -= rhs[(row, col)];
            }
        }
    }
}

impl<const R: usize, const C: usize> Sub for Matrix<R, C> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<const I: usize, const J: usize, const K: usize> 
    Mul<Matrix<J, K>> for Matrix<I, J> {
    type Output = Matrix<I, K>;

    fn mul(self, rhs: Matrix<J, K>) -> Self::Output {
        let mut prod = Matrix::zero();
        for i in 0..I {
            for k in 0..K {
                let mut prod_ik = 0.0;
                for j in 0..J {
                    prod_ik += self[(i, j)] * rhs[(j, k)];
                }
                prod[(i, k)] = prod_ik;
            }
        }
        prod
    }
}

impl<const R: usize, const C: usize> Index<(usize, usize)> for Matrix<R, C> {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.values[row][col]
    }
}

impl<const R: usize, const C: usize> IndexMut<(usize, usize)> for Matrix<R, C> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.values[row][col]
    }
}
