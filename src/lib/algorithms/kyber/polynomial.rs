use crate::algebraic::galois_field::GaloisField;
use crate::algebraic::polynomial::{Polynomial, RingElement};
use crate::algorithms::kyber::byte_array::ByteArray;
use crate::algorithms::kyber::galois_field::GF3329;
use crate::algorithms::kyber::constants::{KYBER_N_VALUE};
use crate::algorithms::kyber::encoder::Encoder;
use crate::algorithms::kyber::ntt::{br7, NTT, ntt_inv_rec, ntt_rec, ZETAS_256};
use crate::algorithms::kyber::utils::poly_coefficients_to_bits;

pub type PolyRQ = Polynomial<GF3329, KYBER_N_VALUE>;

impl NTT for PolyRQ {
    fn inverse_ntt(self) -> Self {
        // Checking if the polynomial is not zero
        if self.is_zero() {
            return PolyRQ::zero();
        }

        let mut coefficients = [GF3329::default(); KYBER_N_VALUE];
        coefficients.copy_from_slice(self.get_coefficients());

        ntt_inv_rec(&mut coefficients, 1, KYBER_N_VALUE / 2);

        coefficients.into()
    }

    fn to_ntt(self) -> Self {
        // Checking if the polynomial is not zero
        if self.is_zero() {
            return PolyRQ::zero();
        }

        let mut coefficients = [GF3329::default(); KYBER_N_VALUE];
        coefficients.copy_from_slice(self.get_coefficients());

        ntt_rec(&mut coefficients, 1, KYBER_N_VALUE / 2);

        coefficients.into()
    }
}

impl PolyRQ {
    pub fn multiply_ntt(&self, other: &Self) -> Self {
        let mut coefficients = [GF3329::default(); KYBER_N_VALUE];

        for i in 0..KYBER_N_VALUE/2 {
            let a_0 = self[2 * i];
            let a_1 = self[2 * i + 1];

            let b_0 = other[2 * i];
            let b_1 = other[2 * i + 1];

            let zeta_index = 2 * br7(i as u8) as usize + 1;
            let zeta = GF3329::from(ZETAS_256[zeta_index]);

            let f_0_hat = a_0.mul(&b_0).add(&a_1.mul(&b_1).mul(&zeta));
            let f_1_hat = a_1.mul(&b_0).add(&a_0.mul(&b_1));

            coefficients[2 * i] = f_0_hat;
            coefficients[2 * i + 1] = f_1_hat;
        }

        coefficients.into()
    }
}

impl Encoder for PolyRQ {
    fn encode(&self, l_value: usize) -> ByteArray {
        let bits = poly_coefficients_to_bits(self, l_value);
        ByteArray::from_bits(bits)
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::kyber::constants::KYBER_N_VALUE;
    use crate::algorithms::kyber::galois_field::GF3329;
    use crate::algorithms::kyber::ntt::NTT;
    use crate::algorithms::kyber::polynomial::PolyRQ;

    #[test]
    pub fn test_to_ntt() {
        let poly = PolyRQ::from_degrees(
            &[0, 1, 2, 128, 210],
            &[1.into(), 2.into(), 3.into(), 9.into(), 10.into()]
        );
        let expected_ntt_coefficients_raw = [
            1298, 2, 3194, 2, 1022, 2, 141, 2, 3221, 2, 1271, 2, 218, 2, 945, 2, 3289, 2, 1203, 2, 1275, 2, 3217, 2, 25, 2, 1138, 2, 474, 2, 689, 2, 238, 2, 925, 2, 2561, 2, 1931, 2, 1530, 2, 2962, 2, 2670, 2, 1822, 2, 2163, 2, 2329, 2, 1886, 2, 2606, 2, 1162, 2, 1, 2, 2237, 2, 2255, 2, 100, 2, 1063, 2, 318, 2, 845, 2, 1693, 2, 2799, 2, 1532, 2, 2960, 2, 457, 2, 706, 2, 1706, 2, 2786, 2, 620, 2, 543, 2, 568, 2, 595, 2, 1267, 2, 3225, 2, 687, 2, 476, 2, 3137, 2, 1355, 2, 1458, 2, 3034, 2, 3200, 2, 1292, 2, 528, 2, 635, 2, 106, 2, 1057, 2, 705, 2, 458, 2, 536, 2, 1634, 2, 629, 2, 1541, 2, 2819, 2, 2680, 2, 3071, 2, 2428, 2, 2349, 2, 3150, 2, 2717, 2, 2782, 2, 3114, 2, 2385, 2, 460, 2, 1710, 2, 1750, 2, 420, 2, 2365, 2, 3134, 2, 2350, 2, 3149, 2, 1117, 2, 1053, 2, 1145, 2, 1025, 2, 1626, 2, 544, 2, 2751, 2, 2748, 2, 2014, 2, 156, 2, 121, 2, 2049, 2, 2158, 2, 12, 2, 728, 2, 1442, 2, 3026, 2, 2473, 2, 1488, 2, 682, 2, 2111, 2, 59, 2, 2177, 2, 3322, 2, 1610, 2, 560, 2, 294, 2, 1876, 2, 1665, 2, 505, 2, 982, 2, 1188, 2, 2764, 2, 2735, 2, 2968, 2, 2531, 2, 1030, 2, 1140, 2, 2214, 2, 3285, 2, 2332, 2, 3167, 2
        ];
        let mut expected_ntt_coefficients = [GF3329::default(); KYBER_N_VALUE];

        for i in 0..KYBER_N_VALUE {
            expected_ntt_coefficients[i] = GF3329::from(expected_ntt_coefficients_raw[i])
        }


        let expected_ntt: PolyRQ = expected_ntt_coefficients.into();

        let poly_ntt = poly.to_ntt();

        assert_eq!(expected_ntt, poly_ntt);
    }


    #[test]
    pub fn test_ntt_inversion() {
        let poly = PolyRQ::from_degrees(
            &[0, 1, 2, 128, 210, 220],
            &[1.into(), 2.into(), 3.into(), 9.into(), 10.into(),7.into()]
        );
        let ntt_poly = poly.clone().to_ntt();
        let from_ntt_poly = ntt_poly.inverse_ntt();

        assert_eq!(from_ntt_poly, poly)
    }
}