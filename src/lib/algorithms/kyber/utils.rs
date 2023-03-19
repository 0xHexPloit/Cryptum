use std::ops::Rem;
use crate::algebraic::galois_field::GaloisField;
use crate::algorithms::kyber::byte_array::ByteArray;
use crate::algorithms::kyber::constants::{KYBER_N_VALUE, KYBER_Q_VALUE};
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


/// This function rounds to the nearest integer with ties going up
pub fn round(value: f64) -> usize {
    (value + 0.5).floor() as usize
}

pub fn compress_d(x: GF3329, d_value: u32) ->  GF3329 {
    let two_pow_d = 2_usize.pow(d_value) ;
    let mut value = round((two_pow_d as f64 / KYBER_Q_VALUE as f64) * x.into_inner() as f64);
    value = value.rem_euclid(two_pow_d);
    GF3329::from(value)
}

pub fn decompress_d(x: GF3329, d_value: u32) ->  GF3329 {
    let two_pow_d = 2_usize.pow(d_value);
    let value = round((KYBER_Q_VALUE as f64 / two_pow_d as f64) * x.into_inner() as f64);


    GF3329::from(value)
}



#[cfg(test)]
mod tests {
    use crate::algorithms::kyber::utils::round;

    #[test]
    fn test_round_should_return_2() {
        let tested_value = 2.22;
        let expected_value = 2;
        let output = round(tested_value);

        assert_eq!(output, expected_value)
    }

    #[test]
    fn test_round_should_return_3() {
        let tested_value = 2.78;
        let expected_value = 3;
        let output = round(tested_value);

        assert_eq!(output, expected_value);
    }

    #[test]
    fn test_round_with_tie() {
        let tested_value = 2.5;
        let expected_value = 3;
        let output = round(tested_value);

        assert_eq!(output, expected_value);
    }
}

