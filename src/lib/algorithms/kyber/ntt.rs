use crate::algorithms::algebraic::galois_field::GaloisField;
use crate::algorithms::kyber::galois_field::GF3329;


pub trait NTT {
    fn inverse_ntt(self) -> Self;
    fn to_ntt(self) -> Self;
}

const INVERSE_OF_2_MOD_Q: GF3329 =  GF3329::new(1665);

/// This array corresponds to the 256th-roots of unity using the following primitive 17
pub const ZETAS_256: [usize; 256] = [
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

pub fn br7(i: u8) -> u8 {
    br::<7>(i)
}

pub fn ntt_inv_rec(poly: &mut [GF3329], zeta_index: u8, layer: usize) {
    if layer != 2{
        // Applying divide and conquer strategy
        let poly_split = poly.split_at_mut(layer);
        ntt_inv_rec(poly_split.0, zeta_index * 2 , layer / 2);
        ntt_inv_rec(poly_split.1, zeta_index * 2 + 1, layer / 2);
    }

    // As zeta is a 256th primitive root of unity if we take u = zeta^u and v = zeta^(256 - u) then
    // one could affirm that u.v [q] = zeta^u * zeta^(256 - u) [q] = zeta^256 [q] = 1.
    let zeta_index_inverse = (256 - br7(zeta_index) as usize).rem_euclid(256);
    let zeta_inverse = GF3329::from(ZETAS_256[zeta_index_inverse]);

    for i in 0..layer {
        let u_plus_v = poly[i].add(&poly[i + layer]);
        let u_minus_v = poly[i].sub(&poly[i + layer]);

        // WARNING: Based on the paper used https://eprint.iacr.org/2021/563.pdf, we obtain 2 * a_o
        // (and 2 * a_1 * zeta) using the formula this is why we have to define to define the inverse of 2 [q]
        poly[i] = INVERSE_OF_2_MOD_Q.mul(&u_plus_v);
        poly[i + layer] = INVERSE_OF_2_MOD_Q.mul(&zeta_inverse.mul(&u_minus_v));
    }
}


pub fn ntt_rec(poly: &mut [GF3329], zeta_index: u8, layer: usize) {
    if layer < 2 {
        return
    }

    let zeta = GF3329::from(ZETAS_256[br7(zeta_index) as usize]);

    for i in 0..layer {
        let t = zeta.mul(&poly[layer + i]);

        poly[i + layer] = poly[i].sub(&t);
        poly[i] = poly[i].add(&t);

    }

    let poly_split = poly.split_at_mut(layer);


    ntt_rec(poly_split.0, zeta_index * 2, layer / 2);
    ntt_rec(poly_split.1, zeta_index * 2 + 1, layer / 2);
}


#[cfg(test)]
mod tests {
    use crate::algorithms::kyber::ntt::{br7};

    #[test]
    fn test_br_7() {
        let tested_value = 1 as u8;
        let expected_value = 64;
        let out = br7(tested_value);
        assert_eq!(out, expected_value);
    }


}


