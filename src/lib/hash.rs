use std::io::Read;
use sha3::{Sha3_512, Shake128, Shake256};
use sha3::digest::{Update, ExtendableOutput, FixedOutput};

pub fn sha_512(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_512::default();
    hasher.update(data);
    hasher.finalize_fixed().to_vec()
}

pub fn shake_128(data: &[u8], length: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; length];
    let mut hasher = Shake128::default();
    hasher.update(data);
    let mut xof_reader = hasher.finalize_xof();
    xof_reader.read(&mut buffer).expect("Xof reader should give some bytes");
    buffer
}

pub fn shake_256(data: &[u8], length: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; length];
    let mut hasher = Shake256::default();
    hasher.update(data);
    let mut xof_reader = hasher.finalize_xof();
    xof_reader.read(&mut buffer).expect("Xof reader should give some bytes");
    buffer
}


#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use crate::hash::{sha_512, shake_128, shake_256};

    #[test]
    fn test_sha_512() {
        let data = b"telecom".as_slice();
        let output = sha_512(data);
        assert_eq!(output, hex!("099fd622c3a5797b980360ab230600ad42ec392d25d68b715827211eca3e2971c9f445e8161ec80dd3c0e4a55d1bb82a5d0da8164b1f8816cbec43cdab8d4e59"))
    }

    #[test]
    fn test_shake_128() {
        let data = b"telecom".as_slice();
        let expected_output_size = 10;
        let output = shake_128(data , expected_output_size);

        assert_eq!(output.len(), expected_output_size);
        assert_eq!(output, hex!("18d71c6c1c2f8edac4e4"))
    }

    #[test]
    fn test_shake_256() {
        let data = b"telecom".as_slice();
        let expected_size = 10;
        let output = shake_256(data, expected_size);

        assert_eq!(output.len(), expected_size);
        assert_eq!(output, hex!("20a3a9e642efb29ceb5b"))
    }
}
