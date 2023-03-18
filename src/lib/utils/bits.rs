use std::cmp::max;

pub fn byte_to_bits(byte_value: u8, bits_arr: &mut [u8; 8]) {
    for i in 0..8 {
        bits_arr[7 - i] = (byte_value >> i) & 0x1
    }
}

pub fn bits_to_byte(bits: &[u8]) -> u8 {
    // Checking length
    if bits.len() != 8 {
        panic!("Invalid bits array. It should have a length of 8!")
    }

    let mut byte: u8 = 0;

    for i in 0..8 {
        let bit = bits[i];

        if bit != 0 && bit != 1 {
            panic!("Invalid value for bit !")
        }

        byte += bits[i] * 2_u8.pow(i as u32)
    }

    byte
}


#[cfg(test)]
mod tests {
    use crate::utils::bits::{bits_to_byte, byte_to_bits};

    #[test]
    fn test_conversion_byte_to_bits() {
        let mut bits = [0u8; 8];
        byte_to_bits(3, &mut bits);
        assert_eq!(bits, [1, 1, 0, 0, 0, 0, 0, 0])
    }

    #[test]
    fn test_conversion_bits_to_byte() {
        let bits = [1, 1, 0, 0, 0, 0, 0, 0];
        let byte = bits_to_byte(&bits);
        let expected_value = 3;
        assert_eq!(byte, expected_value)

    }
}