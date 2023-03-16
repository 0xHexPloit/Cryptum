use crate::algebraic::polynomial::RingElement;
use crate::algebraic::vector::Vector;
use crate::algorithms::kyber::ntt::NTT;
use crate::algorithms::kyber::polynomial::PolyRQ;

pub type VectorRQ = Vector<PolyRQ>;

impl NTT for VectorRQ{
    fn inverse_ntt(self) -> Self {
        let mut coefficients = vec![];

        for i in 0..self.get_n() {
            let poly = self[i].clone();
            coefficients[i] = poly.inverse_ntt();
        }

        coefficients.into()
    }

    fn to_ntt(self) -> Self {
        let mut coefficients = Vec::with_capacity(self.get_n());

        for i in 0..self.get_n() {
            let poly = self[i].clone();
            let poly_ntt = poly.to_ntt();
            coefficients.push(poly_ntt);
        }

        coefficients.into()
    }
}

impl VectorRQ {
    pub fn dot_ntt(&self, other: &Self) -> PolyRQ {
        // Checking that both vectors have the same number of elements
        if self.get_n() != other.get_n() {
            panic!("Vectors don't have the same number of elements")
        }

        let mut poly = PolyRQ::zero();

        for i in 0..self.get_n() {
            let base_multiplication = self[i].multiply_ntt(&other[i]);
            poly = poly.add(&base_multiplication);
        }

        poly
    }
}