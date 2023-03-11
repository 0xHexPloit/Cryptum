use crate::algebraic::galois_field::GaloisField;

pub trait RingElement {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

#[derive(Debug)]
pub struct Polynomial<C, const N: usize>
where C: GaloisField + Default {
    coefficients: [C; N],
    degree: Option<usize>
}

impl <C, const N: usize> From<[C; N]> for Polynomial<C, N> where C: GaloisField + Default + Copy + Clone{
    fn from(value: [C; N]) -> Self {
        let mut degree = value.len() - 1;
        for coeff in value.iter().rev() {
            if coeff.value() == 0 && degree != 0 {
                degree -= 1;
            } else {
                break;
            }
        }

        // In case degree is 0, we should check if we should create the zero polynomial
        let degree =  if degree == 0 && value.get(0).unwrap().value() == 0 {
            None
        } else {
            Some(degree)
        };

        Self {
            coefficients: value,
            degree
        }


    }
}

impl <C, const N: usize>  Polynomial<C, N> where C: GaloisField + Default + Copy + Clone {
    pub fn degree(&self) -> Option<usize> {
        self.degree
    }
    pub fn get_coeff(&self, degree: usize) -> Option<&C> {
        self.coefficients.get(degree)
    }
}


impl <C, const N: usize> RingElement for Polynomial<C, N> where C: GaloisField + Default + Copy + Clone {
    fn zero() -> Self {
        let coeff = [C::default(); N];
        coeff.into()
    }

    fn is_zero(&self) -> bool {
        self.coefficients.iter().all(|coeff| coeff.value() == 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::algebraic::galois_field::GaloisFieldCore;
    use crate::algebraic::polynomial::Polynomial;

    const RING_ORDER: usize = 2;
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
}
