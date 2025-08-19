use std::{fmt::Display, ops::{Index, IndexMut, Mul}};


#[derive(Debug, Clone)]
pub struct Vector {
    comp: Box<[f64]>
}

impl Vector {
    pub const fn new(comp: Box<[f64]>) -> Self {
        Vector { comp }
    }

    pub fn zeros(len: usize) -> Self {
        let comp = vec![0.0; len].into_boxed_slice();
        Vector::new(comp)
    }

    pub const fn len(&self) -> usize { self.comp.len() }

    pub fn dot(&self, rhs: &Self) -> f64 {
        let mut prod = 0.0;
        for (&l, &r) in self.comp.iter().zip(rhs.comp.iter()) {
            prod += l * r;
        }
        prod
    }
}

impl Mul for &Vector {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.comp[index]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.comp[index]
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.comp)
    }
}