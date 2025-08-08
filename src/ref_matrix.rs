
use std::ops::{Index, IndexMut, Mul};
//use crate::macros;

struct RefVector<'a> {
    values: Box<[&'a mut f64]>
}

impl<'a> RefVector<'a> {
    fn new(values: Box<[&'a mut f64]>) -> Self {
        Self { values }
    }
    
    fn len(&self) -> usize {
        self.values.len()
    }
}

impl<'a> Index<usize> for RefVector<'a>{
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        self.values[idx]
    }
}

impl<'a> IndexMut<usize> for RefVector<'a>{
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.values[idx]
    }
}


impl<'a> Mul for &RefVector<'a> {
    type Output = f64;

    /// Dot product
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.len(), rhs.len(), "Vector lengths are not equal");
        let mut prod = 0.0;
        for (&&mut a, &&mut b) in self.values.iter().zip(&rhs.values) {
            prod += a * b;
        }
        prod
    }
}
