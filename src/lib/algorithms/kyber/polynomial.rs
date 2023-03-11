use crate::algebraic::polynomial::Polynomial;
use crate::algorithms::kyber::galois_field::GF3329;
use crate::algorithms::kyber::constants::KYBER_N_VALUE;

pub type PolyRQ = Polynomial<GF3329, KYBER_N_VALUE>;