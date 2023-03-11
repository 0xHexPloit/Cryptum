
pub trait GaloisField {
    fn zero() -> Self;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn value(&self) -> usize;
}

#[derive(Debug, Copy, Clone)]
pub struct GaloisFieldCore<const P: usize>(usize);


impl <const P: usize>From<usize> for GaloisFieldCore<P> {
    fn from(value: usize) -> Self {
        Self (value % P)
    }
}

impl <const P: usize>From<i32> for GaloisFieldCore<P> {
    fn from(value: i32) -> Self {
        let new_value = value % P as i32;
        Self (new_value as usize)
    }
}

impl<const P: usize> GaloisFieldCore<P> {
    pub fn into_inner(self) -> usize {
        self.0
    }
}


impl <const P: usize>Default for GaloisFieldCore<P> {
    fn default() -> Self {
        0.into()
    }
}


impl <const P: usize>GaloisField for GaloisFieldCore<P> {
    fn zero() -> Self {
        0.into()
    }

    fn add(&self, other: &Self) -> Self {
        ((self.0 + other.0) % P).into()
    }

    fn sub(&self, other: &Self) -> Self {
        let signed_value = self.0 as isize;
        let other_signed_value = other.0 as isize;
        let diff = signed_value - other_signed_value;
        let value = (diff  % P as isize);
        (value as usize).into()
    }

    fn mul(&self, other: &Self) -> Self {
        ((self.0 * other.0) % P).into()
    }

    fn value(&self) -> usize {
        self.0
    }
}


#[cfg(test)]
mod tests {
    use crate::algebraic::galois_field::GaloisFieldCore;

    type GF2 = GaloisFieldCore<2>;

    #[test]
    fn test_parsing_negative_number() {
        let expected_value = 1;
        let tested_value = -1;
        let gf_value: GF2 = tested_value.into();
        assert_eq!(gf_value.0, expected_value)
    }
}