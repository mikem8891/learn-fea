use std::{fmt::Display, ops::{Add, Index, IndexMut, Mul}};

#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize> {
    comp: [f64; N],
}

impl<const N: usize> Vector<N> {
    pub const fn new(comp: [f64; N]) -> Self {
        Vector { comp }
    }

    pub const fn zeros() -> Self {
        Vector::new([0.0; N])
    }

    pub const fn len(&self) -> usize { N }

    pub fn dot(&self, rhs: Self) -> f64 {
        let mut prod = 0.0;
        for (l, r) in self.into_iter().zip(rhs) {
            prod += l * r;
        }
        prod
    }
}

impl Vector<2> {
    pub fn cross(&self, rhs: Self) -> f64 {
        self[0] * rhs[1] - self[1] * rhs[0]
    }
}

impl<const N: usize> From<[f64; N]> for Vector<N> {
    fn from(comp: [f64; N]) -> Self {
        Vector::new(comp)
    }
}

impl<const N: usize> From<Vector<N>> for [f64; N] {
    fn from(vec: Vector<N>) -> Self {
        vec.comp
    }
}

impl<const N:usize> Add for Vector<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = std::array::from_fn(|i| self[i] + rhs[i]);
        Vector::new(sum)
    }
}

impl<const N:usize> Mul for Vector<N>{
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl<const N:usize> Mul<f64> for Vector<N>{
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let prod = std::array::from_fn(|i| self[i] * rhs);
        Vector::new(prod)
    }
}

impl<const N: usize> IntoIterator for Vector<N> {
    type Item = f64;

    type IntoIter = std::array::IntoIter<f64, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.comp.into_iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a mut Vector<N> {
    type Item = &'a mut f64;

    type IntoIter = std::slice::IterMut<'a, f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.comp.iter_mut()
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.comp[index]
    }
}

impl<const N: usize> IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.comp[index]
    }
}

impl<const N: usize> Display for Vector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.comp)
    }
}