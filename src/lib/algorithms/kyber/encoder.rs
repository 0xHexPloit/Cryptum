use crate::algorithms::kyber::byte_array::ByteArray;

pub trait Encoder {
    fn encode(&self, l_value: usize) -> ByteArray;
}