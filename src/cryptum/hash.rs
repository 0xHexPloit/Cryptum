use sha3::{Digest, Sha3_512};
use hex_literal::hex;
use sha3::digest::FixedOutput;

pub fn sha_512(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_512::default();
    hasher.update(data);
    hasher.finalize().to_vec()
}


#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use crate::hash::sha_512;

    #[test]
    fn test_sha_512() {
        let data = b"telecom".as_slice();
        let output = sha_512(data);
        assert_eq!(output, hex!("099fd622c3a5797b980360ab230600ad42ec392d25d68b715827211eca3e2971c9f445e8161ec80dd3c0e4a55d1bb82a5d0da8164b1f8816cbec43cdab8d4e59"))
    }
}
