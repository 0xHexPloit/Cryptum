use rand::{RngCore, thread_rng};
use crate::algorithms::utils::bits::{bits_to_byte, byte_to_bits};
use crate::CryptumResult;

#[derive(Debug, PartialEq, Clone)]
pub struct ByteArray {
    values: Vec<u8>
}

impl ByteArray {
    pub fn random(size: usize) -> Self {
        let mut arr = vec![0u8; size];
        let mut rng = thread_rng();
        rng.fill_bytes(&mut arr);
        Self {values: arr}
    }

    pub fn get_bytes(&self) -> &[u8] {
        self.values.as_slice()
    }

    pub fn length(&self) -> usize {
        self.values.len()
    }

    pub fn concat(items: &[&Self]) -> Self {
        let data_length = items.iter().map(|arr| arr.get_bytes().len()).sum();
        let mut data = Vec::with_capacity(data_length);

        for item in items {
            data.extend_from_slice(&item.get_bytes())
        }
        Self {values: data}
    }

    pub fn to_bits(&self) -> Vec<u8> {
        let mut output = vec![];

        for byte in self.get_bytes() {
            let mut bits = [0u8; 8];
            byte_to_bits(*byte, &mut bits);
            output.extend_from_slice(&bits);
        }
        output
    }

    pub fn from_bits(bits: Vec<u8>) -> Self {
        // Checking that bits has the correct length
        if bits.len() % 8 != 0 {
            panic!("bits has an incorrect length")
        }
        let output_length = bits.len() % 8;
        let mut bytes = Vec::with_capacity(output_length);

        for chunk in bits.chunks_exact(8) {
            let byte = bits_to_byte(chunk);
            bytes.push(byte);
        }

        bytes.into()
    }

    pub fn empty() -> Self {
        Self {values: vec![]}
    }

    pub fn slice(&self, start_pos: usize) -> ByteArray {
        let (_, slice) = self.values.split_at(start_pos);
        slice.into()
    }

    pub fn split_at(&self, pos: usize) -> (Self, Self) {
        let (bytes_1, bytes_2) = self.values.split_at(pos);
        (bytes_1.into(), bytes_2.into())
    }

    pub fn to_hex(self) -> String {
        hex::encode(self.values)
    }

    pub fn from_hex(data: String) -> CryptumResult<ByteArray> {
        let data = hex::decode(data)?;
        Ok(data.into())
    }

}

impl From<&[u8]> for ByteArray {
    fn from(value: &[u8]) -> Self {
        Self {
            values: value.to_vec()
        }
    }
}

impl From<u8> for ByteArray {
    fn from(value: u8) -> Self {
        Self {
            values: vec![value]
        }
    }
}

impl From<Vec<u8>> for ByteArray {
    fn from(value: Vec<u8>) -> Self {
        Self {
            values: value
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::byte_array::ByteArray;

    #[test]
    fn test_random_byte_array_creation() {
        let expected_size = 3;
        let arr = ByteArray::random(expected_size);
        assert_eq!(arr.values.len(), expected_size);
    }

    #[test]
    fn test_get_bytes() {
        let expected_size = 2;
        let arr = ByteArray::random(expected_size);
        let bytes = arr.get_bytes();
        assert_eq!(bytes.len(), expected_size)
    }

    #[test]
    fn test_get_bytes_arr_from_bytes_slice() {
        let data = [1, 2].as_slice();
        let bytes_arr = ByteArray::from(data);
        assert_eq!(data, bytes_arr.get_bytes())
    }

    #[test]
    fn test_concat() {
        let expected_size = 3;
        let byte_arr_1 = ByteArray::from([1u8, 2].as_slice());
        let byte_arr_2 = ByteArray::from(4);
        let concat_arr = ByteArray::concat(&[&byte_arr_1, &byte_arr_2]);

        assert_eq!(concat_arr.get_bytes().len(), expected_size);
        assert_eq!(concat_arr.get_bytes(), [1, 2, 4])
    }

    #[test]
    fn test_to_bits() {
        let data = ByteArray::from([2, 3].as_slice());
        let bits = data.to_bits();
        assert_eq!(bits, [0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0])
    }

    #[test]
    fn test_length() {
        let data = [1, 2, 4];
        let bytes_array = ByteArray::from(data.to_vec());
        let expected_length = 3;
        assert_eq!(bytes_array.length(), expected_length)
    }
}
