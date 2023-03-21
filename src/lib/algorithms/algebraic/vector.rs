use std::ops::{Add, Index};
use crate::algorithms::algebraic::polynomial::RingElement;

#[derive(Debug)]
pub struct Vector<C: RingElement> {
    coefficients: Vec<C>,
    n: usize
}

impl <C: RingElement> From<Vec<C>> for Vector<C> {
    fn from(value: Vec<C>) -> Self {
        let length = value.len();
        Self {coefficients: value, n: length}

    }
}

impl <C: RingElement>Index<usize> for Vector<C> {
    type Output = C;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coefficients[index]
    }
}

impl <C: RingElement>  Vector<C> {
    pub fn get_n(&self) -> usize {
        self.n
    }
}

impl <C: RingElement + Clone>Add for Vector<C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Checking that both vectors have the same number of coefficients
        if self.get_n() != rhs.get_n() {
            panic!("Vectors don't have the same number of coefficients. Cannot perform addition")
        }

        let mut polynomials: Vec<C> = Vec::with_capacity(self.get_n());

        for i in 0..self.get_n() {
            let poly = &self[i].add(&rhs[i]);
            polynomials.push(poly.clone())
        }

        polynomials.into()
    }
}