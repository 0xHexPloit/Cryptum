pub fn byte_to_bits(byte_value: u8, bits_arr: &mut [u8; 8]) {
    for i in 0..8 {
        bits_arr[7 - i] = (byte_value >> i) & 0x1
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::bytes::byte_to_bits;

    #[test]
    pub fn test_conversion_byte_to_bits() {
        let mut bits = [0u8; 8];
        byte_to_bits(3, &mut bits);
        assert_eq!(bits, [0, 0, 0, 0, 0, 0, 1, 1])
    }
}