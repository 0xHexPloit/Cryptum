use crate::byte_array::ByteArray;
use crate::hash::sha_512;

const PLAINTEXT_SIZE_IN_BYTES: usize = 32;

pub struct KyberCPAPKE<const V: usize> {
    n: usize,
    k: usize,
    q: usize,
    eta_1: usize,
    eta_2: usize,
    d_u: usize,
    d_v: usize
}


impl KyberCPAPKE<512> {
    pub fn init() -> Self {
        Self {
            n: 256,
            k: 2,
            q: 3329,
            eta_1: 3,
            eta_2: 2,
            d_u: 10,
            d_v: 4
        }
    }
}

impl KyberCPAPKE<768> {
    pub fn init() -> Self {
        Self {
            n: 256,
            k: 3,
            q: 3329,
            eta_1: 2,
            eta_2: 2,
            d_u: 10,
            d_v: 4
        }
    }
}

impl KyberCPAPKE<1024> {
    pub fn init() -> Self {
        Self {
            n: 256,
            k: 4,
            q: 3329,
            eta_1: 2,
            eta_2: 2,
            d_u: 11,
            d_v: 5
        }
    }
}


impl <const N: usize> KyberCPAPKE<N> {
    fn g(&self, seed: ByteArray) -> (ByteArray, ByteArray) {
        let hash = sha_512(seed.get_bytes());
        let data = hash.split_at(PLAINTEXT_SIZE_IN_BYTES);
        (ByteArray::from(data.0), ByteArray::from(data.1))
    }

    pub fn keygen(&self) -> (ByteArray, ByteArray) {
        let d = ByteArray::random(PLAINTEXT_SIZE_IN_BYTES);
        let (rho, sigma) = self.g(d);



        (ByteArray::random(2), ByteArray::random(2))
    }
}

pub type KyberCPAPKE512 = KyberCPAPKE<512>;
pub type KyberCPAPKE768 = KyberCPAPKE<768>;
pub type KyberCPAPKE1024 = KyberCPAPKE<1024>;


#[cfg(test)]
mod tests {
    use crate::algorithms::kyber::cpapke::{KyberCPAPKE512, PLAINTEXT_SIZE_IN_BYTES};
    use crate::byte_array::ByteArray;

    #[test]
    fn test_g() {
        let seed = ByteArray::random(PLAINTEXT_SIZE_IN_BYTES);
        let kyber = KyberCPAPKE512::init();
        let (rho, sigma) = kyber.g(seed);
        assert_eq!(rho.get_bytes().len(), PLAINTEXT_SIZE_IN_BYTES);
        assert_eq!(sigma.get_bytes().len(), PLAINTEXT_SIZE_IN_BYTES);
    }
}