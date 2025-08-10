use std::ops::{Index, IndexMut};


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