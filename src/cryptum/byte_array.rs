use rand::{RngCore, thread_rng};

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
}

impl From<&[u8]> for ByteArray {
    fn from(value: &[u8]) -> Self {
        Self {
            values: value.to_vec()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::byte_array::ByteArray;

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
}
