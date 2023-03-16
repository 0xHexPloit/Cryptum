use crate::algebraic::matrix::Matrix;
use crate::algorithms::kyber::polynomial::PolyRQ;
use crate::algorithms::kyber::vector::VectorRQ;

pub type MatrixRQ = Matrix<PolyRQ>;

impl MatrixRQ {
    pub fn multiply_vec(&self, other: &VectorRQ) -> VectorRQ {
        let mut polynomials = Vec::with_capacity(other.get_n());
        let matrix_shape = self.get_shape();

        for i in 0..matrix_shape.0 {
            let row = self.get_row(i as usize);
            let polynomial = other.dot_ntt(&VectorRQ::from(row.clone()));
            polynomials.push(polynomial);
        }

        polynomials.into()
    }
}