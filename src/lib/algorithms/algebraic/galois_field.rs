
pub trait GaloisField {
    fn is_zero(&self) -> bool;
    fn zero() -> Self;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn value(&self) -> usize;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GaloisFieldCore<const P: usize>(usize);

impl <const P: usize>GaloisFieldCore<P> {
    pub const fn new(val: usize) -> Self {
        Self(val.rem_euclid(P))
    }
    pub fn into_inner(self) -> usize {
        self.0
    }
}


impl <const P: usize>From<usize> for GaloisFieldCore<P> {
    fn from(value: usize) -> Self {
        Self (value.rem_euclid(P))
    }
}

impl <const P: usize>From<i32> for GaloisFieldCore<P> {
    fn from(value: i32) -> Self {
        Self (value.rem_euclid(P as i32) as usize)
    }
}


impl <const P: usize>Default for GaloisFieldCore<P> {
    fn default() -> Self {
        0.into()
    }
}


impl <const P: usize>GaloisField for GaloisFieldCore<P> {
    fn is_zero(&self) -> bool {
        self.0 == 0
    }

    fn zero() -> Self {
        0.into()
    }

    fn add(&self, other: &Self) -> Self {
        ((self.0 + other.0).rem_euclid(P)).into()
    }

    fn sub(&self, other: &Self) -> Self {
        let self_value = self.value() as i32;
        let other_value = other.value() as i32;

        (self_value - other_value).into()
    }


    fn mul(&self, other: &Self) -> Self {
        ((self.0 * other.0).rem_euclid(P)).into()
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