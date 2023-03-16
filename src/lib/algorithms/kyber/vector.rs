use crate::algebraic::galois_field::GaloisField;
use crate::algebraic::polynomial::RingElement;
use crate::algebraic::vector::Vector;
use crate::algorithms::kyber::ntt::NTT;
use crate::algorithms::kyber::polynomial::PolyRQ;

type VectorRQ = Vector<PolyRQ>;

impl NTT for VectorRQ{
    fn inverse_ntt(self) -> Self {
        let coefficients = vec![];

        for i in 0..self.get_num_coefficients() {
            coefficients[i] = self[i].inverse_ntt()
        }

        coefficients.into()
    }

    fn to_ntt(self) -> Self {
        let coefficients = vec![];

        for i in 0..self.get_num_coefficients() {
            coefficients[i] = self[i].to_ntt()
        }

        coefficients.into()
    }
}