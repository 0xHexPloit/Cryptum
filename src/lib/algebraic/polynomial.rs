use std::cmp::{max};
use std::ops::{Index};
use crate::algebraic::galois_field::GaloisField;

pub trait RingElement {
    fn degree(&self) -> Option<usize>;
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
    fn mul(&self, other: &Self) -> Self;
    fn remainder(&self, divisor: &Self) -> Self;
    fn add(&self, other: &Self) -> Self;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<C, const N: usize>
where C: GaloisField + Default {
    coefficients: [C; N],
    degree: Option<usize>
}

impl <C, const N: usize> From<[C; N]> for Polynomial<C, N> where C: GaloisField + Default + Copy + Clone + From<i32>{
    fn from(value: [C; N]) -> Self {
        let degree= Self::find_degree(&value);
        Self {
            coefficients: value,
            degree
        }
    }
}



impl <C, const N: usize>  Polynomial<C, N> where C: GaloisField + Default + Copy + Clone  + From<i32> {
    pub fn from_degrees(degrees: &[usize], coefficients: &[C]) -> Self {
        // Checking that degrees and coefficients have the same length
        if degrees.len() != coefficients.len() {
            panic!("'degrees' and 'coefficients' don't have the same length")
        }

        let mut inner_coefficients = [C::default(); N];

        for (degree, coeff) in degrees.iter().zip(coefficients.iter()) {
            if *degree >= N {
                panic!("Degree {} is to high ! Cannot build the polynomial!", degree);
            }
            inner_coefficients[*degree] = *coeff;
        }

        inner_coefficients.into()
    }

    fn find_degree(coefficients: &[C]) -> Option<usize> {
        let mut degree = coefficients.len() - 1;
        for coeff in coefficients.iter().rev() {
            if coeff.value() == 0 && degree != 0 {
                degree -= 1;
            } else {
                break;
            }
        }

        // In case degree is 0, we should check if we should create the zero polynomial
        if degree == 0 && coefficients.get(0).unwrap().value() == 0 {
            None
        } else {
            Some(degree)
        }
    }

    pub fn get_coefficients(&self) -> &[C] {
        self.coefficients.as_slice()
    }


    fn poly_euclidean_division(poly: &[C], divisor: &[C]) -> Vec<C> {
        let poly_degree = Self::find_degree(poly);
        let divisor_degree = Self::find_degree(divisor);

        let mut poly_copy = poly.to_vec();

        // Checking that divisor is not zero otherwise panic
        if divisor_degree.is_none() {
            panic!("Divisor cannot be the zero polynomial")
        }

        // Checking degrees
        if poly_degree.is_none() || poly_degree < divisor_degree {
            return poly_copy;
        }

        let mut poly_degree = poly_degree.unwrap();
        let divisor_degree = divisor_degree.unwrap();


        loop {
            let poly_coefficient = poly_copy[poly_degree];

            // Determining lambda_value and lambda_degree
            let neg = -1 * poly_coefficient.value() as i32;
            let lambda_value: C = neg.into();
            let lambda_degree = poly_degree - divisor_degree;


            for j in 0..divisor.len() {
                let mul_value = lambda_value.mul(&divisor[j]);
                poly_copy[j + lambda_degree] = poly_copy[j + lambda_degree].add(&mul_value);
            }

            poly_degree = Self::find_degree(&poly_copy).unwrap();

            if poly_degree < divisor_degree {
                break;
            }
        }
        poly_copy
    }
}


impl <C, const N: usize> RingElement for Polynomial<C, N> where C: GaloisField + Default + Copy + Clone + From<i32> {
    fn degree(&self) -> Option<usize> {
        self.degree
    }

    fn zero() -> Self {
        let coeff = [C::default(); N];
        coeff.into()
    }

    fn is_zero(&self) -> bool {
        self.coefficients.iter().all(|coeff| coeff.value() == 0)
    }

    fn mul(&self, other: &Self) -> Self {
        // Checking if self are other is the zero polynomial
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }

        let self_degree = self.degree().unwrap();
        let other_degree = other.degree().unwrap();

        let mut coefficients = vec![C::default(); N * 2];

        for i in 0..=self_degree {
            for j in 0..=other_degree {
                let f_coefficient = self[i];
                let g_coefficient = other[j];

                if f_coefficient.is_zero() || g_coefficient.is_zero() {
                    continue
                }

                let mul_value = f_coefficient.mul(&g_coefficient);

                coefficients[i + j] = coefficients[i + j].add(&mul_value);
            }
        }

        if self_degree + other_degree < N {
            let mut buffer = [C::default(); N];
            buffer.copy_from_slice(&coefficients[0..N]);
            return buffer.into();
        }

        // We have to apply a polynomial euclidean division so that the coefficient array represents
        // a polynomial of the ring
        let mut divisor = vec![C::from(0); N+1];
        divisor[0] = 1.into();
        divisor[N] = 1.into();

        let remainder = Self::poly_euclidean_division(&coefficients, &divisor);
        let mut buffer = [C::from(0); N];
        buffer.copy_from_slice(&remainder[0..N]);

        buffer.into()
    }

    fn remainder(&self, divisor: &Self) -> Self {
        let remainder_coefficients = Self::poly_euclidean_division(&self.coefficients, &divisor.coefficients);
        let mut buffer = [C::default(); N];
        buffer.copy_from_slice(&remainder_coefficients[0..N]);
        buffer.into()
    }

    fn add(&self, other: &Self) -> Self {
        // Checking if self or other is the zero polynomial
        if self.is_zero() {
            return other.clone();
        }
        if other.is_zero() {
            return self.clone();
        }

        let mut coefficients = [C::default(); N];
        let max_degree = max(self.degree.unwrap(), other.degree.unwrap());

        for i in 0..=max_degree {
            coefficients[i] = self.coefficients[i].add(&other.coefficients[i]);
        }

        coefficients.into()
    }
}

impl <C, const N: usize>Index<usize> for Polynomial<C, N> where  C: GaloisField + Default + Copy + Clone {
    type Output = C;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= N {
            panic!("Invalid index!")
        }
        self.coefficients.get(index).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use crate::algebraic::galois_field::{GaloisField, GaloisFieldCore};
    use crate::algebraic::polynomial::{Polynomial, RingElement};

    const RING_ORDER: usize = 4;
    type GF7 = GaloisFieldCore<7>;
    type Poly = Polynomial<GF7, RING_ORDER>;

    #[test]
    fn test_creating_zero_polynomial() {
        let coeff = [
            GF7::default();
            RING_ORDER
        ];
        let poly = Poly::from(coeff);

        assert_eq!(poly.degree, None)
    }

    #[test]
    fn test_creating_non_zero_polynomial() {
        let mut coeff = [GF7::default(); RING_ORDER];
        coeff[1] = 1.into();
        let poly = Poly::from(coeff);

        assert_eq!(poly.degree, Some(1))
    }

    #[test]
    fn test_from_degrees() {
        let expected_degree = 1;

        let poly = Poly::from_degrees(
            &[0, 1],
            &[1.into(), 1.into()]
        );

        assert_eq!(poly.degree, Some(expected_degree));
        assert_eq!(poly[0], GF7::from(1));
        assert_eq!(poly[1], GF7::from(1));
    }

    #[test]
    fn test_multiplication_with_0_polynomial() {
        let f_poly = Poly::zero();
        let g_poly = Poly::from_degrees(
            &[0, 1],
            &[1.into(), 1.into()]
        );
        let out_poly = f_poly.mul(&g_poly);

        assert_eq!(out_poly.is_zero(), true);

        let out_poly = g_poly.mul(&f_poly);

        assert_eq!(out_poly.is_zero(), true);
    }

    #[test]
    fn test_multiplication_with_no_exceed() {
        let f_poly = Poly::from_degrees(
            &[0],
            &[2.into()]
        );
        let g_poly = Poly::from_degrees(
            &[0, 1],
            &[1.into(), 2.into()]
        );
        let out_poly = f_poly.mul(&g_poly);

        let expected_degree = 1;

        assert_eq!(out_poly.degree, Some(expected_degree));
        assert_eq!(out_poly[0], 2.into());
        assert_eq!(out_poly[1], 4.into());
    }

    #[test]
    fn test_find_zero_polynomial_degree() {
        let polynomial = [0.into(); 3];
        let degree = Poly::find_degree(&polynomial);
        assert!(degree.is_none())
    }

    #[test]
    fn test_find_non_zero_polynomial_degree() {
        let mut polynomial = [0.into(); 8];
        polynomial[1] = 1.into();
        let degree = Poly::find_degree(&polynomial);

        let expected_degree = 1;

        assert_eq!(degree, Some(expected_degree))
    }

    #[test]
    fn test_poly_euclidean_division() {
        let mut dividend = [0.into(); 6];
        dividend[5] = 5.into();

        let mut divisor = [0.into(); 5];
        divisor[0] = 1.into();
        divisor[4] = 1.into();

        let remainder = Poly::poly_euclidean_division(&dividend, &divisor);
        let degree = Poly::find_degree(&remainder);

        let expected_degree = 1;
        assert_eq!(degree, Some(expected_degree));
        assert_eq!(remainder[1], 2.into());
        for (idx, coefficient) in remainder.iter().enumerate() {
            if idx != 1 {
                assert!(coefficient.is_zero())
            }
        }
    }

    #[test]
    fn test_poly_euclidean_division_complex() {
        let mut dividend = [0.into(); 5];
        dividend[4] = 4.into();
        dividend[1] = 1.into();
        dividend[0] = 2.into();

        let mut divisor = [0.into(); 3];
        divisor[2] = 1.into();
        divisor[0] = 2.into();

        let remainder = Poly::poly_euclidean_division(&dividend, &divisor);

        let expected_degree = 1;
        assert_eq!(Poly::find_degree(&remainder), Some(expected_degree));
        assert_eq!(remainder[0], 4.into());
        assert_eq!(remainder[1], 1.into());

        for i in 2..remainder.len() {
            assert!(remainder[i].is_zero())
        }
    }

    #[test]
    fn test_multiplication_with_exceed() {
        let f_poly = Poly::from_degrees(
            &[2],
            &[2.into()]
        );
        let g_poly = Poly::from_degrees(
            &[3],
            &[3.into()]
        );
        let out_poly = f_poly.mul(&g_poly);


        let expected_degree = 1;
        let expected_coefficient = 1;

        assert_eq!(out_poly.degree, Some(expected_degree));
        assert_eq!(out_poly[1], expected_coefficient.into());
    }

    #[test]
    fn test_polynomial_addition() {
        let f_poly = Poly::from_degrees(
            &[0, 1, 2],
            &[1.into(), 1.into(), 1.into()]
        );
        let g_poly = Poly::from_degrees(
            &[0, 1],
            &[1.into(), 1.into()]
        );
        let out_poly = f_poly.add(&g_poly);

        let expected_degree = 2;

        assert_eq!(out_poly.degree, Some(expected_degree));
        assert_eq!(out_poly[0], 2.into());
        assert_eq!(out_poly[1], 2.into());
    }
 }
