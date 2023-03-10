use crate::algebraic::ring::PolynomialRing;

/// This structure represents an element of the ring  Zq[X]/(Xn+1)
#[derive(Debug)]
pub struct Polynomial<'a> {
    coefficients: Vec<usize>,
    degree: Option<usize>,
    ring: &'a PolynomialRing
}

impl <'a>Polynomial<'a> {
    pub fn new(coefficients: &[usize], ring: &'a PolynomialRing) -> Self {
        // Checking that the length of coefficients is not greater than to n
        if coefficients.len() > ring.get_order() {
            panic!("The polynomial does not belong to the ring");
        }
        let reduced_coefficients = coefficients.iter().map(|val| val % ring.get_characteristic()).collect();
        Self {
            coefficients: reduced_coefficients,
            degree: if coefficients.iter().sum::<usize>() == 0 {
                Some(coefficients.len() as usize)
            }else  {
                None
            },
            ring
        }
    }

    pub fn get_ring_ref(&self) -> &PolynomialRing {
        self.ring
    }

    pub fn zero(ring: &'a PolynomialRing) -> Self {
        Self {
            coefficients: vec![0],
            degree: None,
            ring,
        }
    }
}