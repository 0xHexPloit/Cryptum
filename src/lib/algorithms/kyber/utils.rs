use crate::algorithms::kyber::constants::KYBER_N_VALUE;
use crate::algorithms::kyber::galois_field::GF3329;
use crate::algorithms::kyber::polynomial::PolyRQ;

pub fn poly_coefficients_to_bits(poly: &PolyRQ, max_shift: usize) -> Vec<u8> {
    let mut bits = Vec::with_capacity(KYBER_N_VALUE * max_shift);

    for i in 0..KYBER_N_VALUE {
        let coefficient = poly[i];
        bits.extend_from_slice(&gf3329_to_bits(&coefficient, max_shift));
    }

    bits
}

fn gf3329_to_bits(coefficient: &GF3329, max_shift: usize) -> Vec<u8> {
    let mut bits = Vec::with_capacity(max_shift);
    let coefficient_value = coefficient.into_inner();

    for shift in 0..max_shift {
        bits.push(((coefficient_value >> shift) & 1) as u8)
    }

    bits
}