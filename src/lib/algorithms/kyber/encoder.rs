use crate::algorithms::kyber::byte_array::ByteArray;

pub trait Encoder {
    fn encode(&self, l_value: usize) -> ByteArray;
}

pub trait Decoder {
    fn decode(bytes: ByteArray, l_value: u8) -> Self;
}