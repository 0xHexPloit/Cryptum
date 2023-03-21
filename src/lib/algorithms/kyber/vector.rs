use crate::algorithms::algebraic::polynomial::RingElement;
use crate::algorithms::algebraic::vector::Vector;
use crate::algorithms::byte_array::ByteArray;
use crate::algorithms::kyber::compress::{Compress, Decompress};
use crate::algorithms::kyber::encoder::{Decoder, Encoder};
use crate::algorithms::kyber::ntt::NTT;
use crate::algorithms::kyber::polynomial::PolyRQ;

pub type VectorRQ = Vector<PolyRQ>;

impl NTT for VectorRQ{
    fn inverse_ntt(self) -> Self {
        let mut coefficients: Vec<PolyRQ> = Vec::with_capacity(self.get_n());

        for i in 0..self.get_n() {
            let poly = self[i].clone();
            coefficients.push(poly.inverse_ntt());
        }

        coefficients.into()
    }

    fn to_ntt(self) -> Self {
        let mut coefficients = Vec::with_capacity(self.get_n());

        for i in 0..self.get_n() {
            let poly = self[i].clone();
            let poly_ntt = poly.to_ntt();
            coefficients.push(poly_ntt);
        }

        coefficients.into()
    }
}

impl Encoder for VectorRQ {
    fn encode(&self, l_value: usize) -> ByteArray {
        let mut bytes = ByteArray::empty();

        for i in 0..self.get_n() {
            let poly = &self[i];
            let poly_bytes = poly.encode(l_value);
            bytes = ByteArray::concat(&[&bytes, &poly_bytes])
        }

        bytes
    }
}

impl Decoder for VectorRQ {
    fn decode(bytes: ByteArray, l_value: u8) -> Self {
        // Checking length of bytes
        let bytes_length = bytes.length();

        if bytes_length % (32 * l_value as usize) != 0 {
            panic!("bytes is not a multiple of {}", 32 * l_value)
        }

        let mut polynomials = Vec::with_capacity(bytes_length / (32 * l_value as usize));

        for chunk in bytes.get_bytes().chunks_exact(32 * l_value as usize) {
            polynomials.push(PolyRQ::decode(chunk.into(), l_value))
        }


        polynomials.into()
    }
}


impl Compress for VectorRQ {
    fn compress(self, d_value: u32) -> Self {
        let mut polynomials = Vec::with_capacity(self.get_n());

        for i in 0..self.get_n() {
            let poly = &self[i];
            polynomials.push(poly.clone().compress(d_value))
        }


        polynomials.into()
    }
}

impl Decompress for VectorRQ {
    fn decompress(self, d_value: u32) -> Self {
        let mut polynomials = Vec::with_capacity(self.get_n());

        for i in 0..self.get_n() {
            let poly = &self[i];
            polynomials.push(poly.clone().decompress(d_value))
        }

        polynomials.into()
    }
}


impl VectorRQ {
    pub fn dot_ntt(&self, other: &Self) -> PolyRQ {
        // Checking that both vectors have the same number of elements
        if self.get_n() != other.get_n() {
            panic!("Vectors don't have the same number of elements")
        }

        let mut poly = PolyRQ::zero();

        for i in 0..self.get_n() {
            let base_multiplication = self[i].multiply_ntt(&other[i]);
            poly = poly.add(&base_multiplication);
        }

        poly
    }
}