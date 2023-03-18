use crate::algebraic::galois_field::GaloisField;
use crate::algorithms::kyber::byte_array::ByteArray;
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


pub fn decode_l(bytes: ByteArray, l_value: u8) -> PolyRQ {
    // Checking length of bytes
    let bytes_length = bytes.length();
    let expected_length = 32 * l_value as usize;
    if bytes_length != expected_length {
        panic!("Invalid length for 'bytes'. Expected {} found {}", expected_length, bytes_length)
    }

    let bits = bytes.to_bits();
    let mut coefficients = [GF3329::default(); KYBER_N_VALUE];

    for i in 0..KYBER_N_VALUE {
        let mut coefficient = GF3329::default();

        for j in 0..l_value {
            let value = bits[i * l_value as usize + j as usize] as u16 * 2_u16.pow(j as u32);
            coefficient = coefficient.add(&GF3329::from(value as usize))
        }

        coefficients[i] = coefficient
    }

    coefficients.into()
}