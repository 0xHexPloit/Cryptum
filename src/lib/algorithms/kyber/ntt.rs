use crate::algebraic::galois_field::GaloisField;
use crate::algebraic::polynomial::RingElement;
use crate::algorithms::kyber::constants::KYBER_N_VALUE;
use crate::algorithms::kyber::galois_field::GF3329;
use crate::algorithms::kyber::polynomial::PolyRQ;
use crate::algorithms::kyber::matrix::MatrixRQ;

/// This array corresponds to the 256th-roots of unity using the following primitive 17
const ZETAS_256: [usize; 256] = [
    1, 17, 289, 1584, 296, 1703, 2319, 2804, 1062, 1409, 650, 1063, 1426, 939, 2647, 1722, 2642,
    1637, 1197, 375, 3046, 1847, 1438, 1143, 2786, 756, 2865, 2099, 2393, 733, 2474, 2110, 2580,
    583, 3253, 2037, 1339, 2789, 807, 403, 193, 3281, 2513, 2773, 535, 2437, 1481, 1874, 1897,
    2288, 2277, 2090, 2240, 1461, 1534, 2775, 569, 3015, 1320, 2466, 1974, 268, 1227, 885, 1729,
    2761, 331, 2298, 2447, 1651, 1435, 1092, 1919, 2662, 1977, 319, 2094, 2308, 2617, 1212, 630,
    723, 2304, 2549, 56, 952, 2868, 2150, 3260, 2156, 33, 561, 2879, 2337, 3110, 2935, 3289, 2649,
    1756, 3220, 1476, 1789, 452, 1026, 797, 233, 632, 757, 2882, 2388, 648, 1029, 848, 1100, 2055,
    1645, 1333, 2687, 2402, 886, 1746, 3050, 1915, 2594, 821, 641, 910, 2154, 3328, 3312, 3040,
    1745, 3033, 1626, 1010, 525, 2267, 1920, 2679, 2266, 1903, 2390, 682, 1607, 687, 1692, 2132,
    2954, 283, 1482, 1891, 2186, 543, 2573, 464, 1230, 936, 2596, 855, 1219, 749, 2746, 76, 1292,
    1990, 540, 2522, 2926, 3136, 48, 816, 556, 2794, 892, 1848, 1455, 1432, 1041, 1052, 1239, 1089,
    1868, 1795, 554, 2760, 314, 2009, 863, 1355, 3061, 2102, 2444, 1600, 568, 2998, 1031, 882,
    1678, 1894, 2237, 1410, 667, 1352, 3010, 1235, 1021, 712, 2117, 2699, 2606, 1025, 780, 3273,
    2377, 461, 1179, 69, 1173, 3296, 2768, 450, 992, 219, 394, 40, 680, 1573, 109, 1853, 1540,
    2877, 2303, 2532, 3096, 2697, 2572, 447, 941, 2681, 2300, 2481, 2229, 1274, 1684, 1996, 642,
    927, 2443, 1583, 279, 1414, 735, 2508, 2688, 2419, 1175,
];

fn br<const K: u8>(i: u8) -> u8 {
    let mask = (1 << K) - 1; // compute the mask with k bits set to 1
    let bin_i = format!("{:0width$b}", i & mask, width = K as usize); // convert the masked value to binary string with leading zeros
    u8::from_str_radix(&bin_i.chars().rev().collect::<String>(), 2).unwrap() // reverse the binary string and convert it back to integer
}

fn br7(i: u8) -> u8 {
    br::<7>(i)
}


pub fn ntt(poly: &PolyRQ) -> PolyRQ {
    // Checking if poly is the zero polynomial
    if poly.is_zero() {
        return PolyRQ::zero();
    }

    let mut coefficients = [GF3329::default(); KYBER_N_VALUE];

    for i in 0..KYBER_N_VALUE/2{
        let mut f_two_i_hat = GF3329::zero();
        let mut f_two_i_plus_one_hat = GF3329::zero();

        for j in 0..KYBER_N_VALUE/2 {
            let zeta_index = ((2 as usize * br7(i as u8) as usize + 1 as usize) * j).rem_euclid(KYBER_N_VALUE);
            let zeta: GF3329 = ZETAS_256.get(zeta_index).unwrap().clone().into();

            let first_poly_coeff = poly[2 as usize * j];
            let second_poly_coeff = poly[2 as usize * j + 1 as usize];

            let first_coeff =  first_poly_coeff.mul(&zeta);
            let second_coeff = second_poly_coeff.mul(&zeta);

            f_two_i_hat = f_two_i_hat.add(&first_coeff);
            f_two_i_plus_one_hat = f_two_i_plus_one_hat.add(&second_coeff);
        }

        coefficients[2 as usize * i] = f_two_i_hat;
        coefficients[2 as usize * i + 1 as usize] = f_two_i_plus_one_hat;
    }
    coefficients.into()
}


pub fn ntt_matrix(matrix: &mut MatrixRQ) {
    let matrix_shape = matrix.get_shape();

    for i in 0..matrix_shape.0 {
        for j in 0..matrix_shape.1 {
            matrix.set(i, j, ntt(matrix.get_element(i, j)))
        }
    }

}

pub fn ntt_basecase_multiplication(f_hat: &PolyRQ, g_hat: &PolyRQ) -> PolyRQ {
    // Checking if one of the two polynomials (at least) is the zero polynomial as the result
    // would be the zero polynomial
    if f_hat.is_zero() || g_hat.is_zero() {
        return PolyRQ::zero()
    }

    let mut coefficients = [GF3329::zero(); KYBER_N_VALUE];

    for i in 0..(KYBER_N_VALUE - 1)/2 {
        let zeta_index = (2 * br7(i as u8) as usize + 1).rem_euclid(KYBER_N_VALUE);
        let zeta = GF3329::from(*ZETAS_256.get(zeta_index).unwrap());

        let f_hat_two_i = f_hat[2 * i];
        let f_hat_two_i_plus_one = f_hat[2 * i + 1];

        let g_hat_two_i = g_hat[2 * i];
        let g_hat_two_i_plus_one = g_hat[2 * i + 1];

        let h_0 = f_hat_two_i
            .mul(&g_hat_two_i)
            .add(&f_hat_two_i_plus_one.mul(&g_hat_two_i_plus_one).mul(&zeta));

        let h_1 = f_hat_two_i
            .mul(&g_hat_two_i_plus_one)
            .add(&g_hat_two_i.mul(&f_hat_two_i_plus_one));

        coefficients[2 * i] = h_0;
        coefficients[2 * i + 1] = h_1;
    }

    coefficients.into()
}


pub fn ntt_matrix_product(matrix_a: &MatrixRQ, matrix_b: &MatrixRQ) -> MatrixRQ {
    // Checking that matrix product is possible
    let matrix_a_shape = matrix_a.get_shape();
    let matrix_b_shape = matrix_b.get_shape();

    if matrix_a_shape.1 != matrix_b_shape.0 {
        panic!("Cannot perform matrix multiplication");
    }

    let mut matrix_data =  Vec::with_capacity(matrix_a_shape.0 as usize);

    for i in 0..matrix_a_shape.0 {
        let mut  row_data = Vec::with_capacity(matrix_b_shape.1 as usize);

        for j in 0..matrix_b_shape.1 {
            let mut cell_poly = PolyRQ::zero();
            for k in 0..matrix_b_shape.0 {
                let first_poly = matrix_a.get_element(i, k);
                let second_poly = matrix_b.get_element(k, j);
                let result_poly = ntt_basecase_multiplication(first_poly, second_poly);
                cell_poly = cell_poly.add(&result_poly);
            }
            row_data.push(cell_poly) ;
        }

        matrix_data.push(row_data);
    }
    matrix_data.into()
}



#[cfg(test)]
mod tests {
    use crate::algorithms::kyber::ntt::br7;

    #[test]
    fn test_br_7() {
        let tested_value = 1 as u8;
        let expected_value = 64;
        let out = br7(tested_value);
        assert_eq!(out, expected_value);
    }
}


