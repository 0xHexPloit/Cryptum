use std::ops::Index;
use crate::algebraic::galois_field::GaloisField;

pub struct Vector<C: GaloisField> {
    coefficients: Vec<C>,
    num_coefficients: usize
}

impl <C: GaloisField> From<Vec<C>> for Vector<C> {
    fn from(value: Vec<C>) -> Self {
        let length = value.len();
        Self {coefficients: value, num_coefficients: length}

    }
}

impl <C: GaloisField>Index<usize> for Vector<C> {
    type Output = C;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coefficients[index]
    }
}

impl <C: GaloisField>  Vector<C> {
    pub fn get_num_coefficients(&self) -> usize {
        self.num_coefficients
    }
}