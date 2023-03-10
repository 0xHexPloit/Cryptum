use crate::algebraic::matrix::PolynomialMatrix;
use crate::byte_array::ByteArray;
use crate::hash::{sha_512, shake_128};
use crate::algebraic::ring::PolynomialRing;
use crate::algorithms::kyber::constants::{KYBER_N_VALUE, KYBER_N_VALUE_IN_BYTES, KYBER_Q_VALUE, KYBER_XOF_DEFAULT_BYTES_STREAM_SIZE};

pub struct KyberCPAPKE<const V: usize> {
    ring: PolynomialRing,
    k: u8,
    eta_1: usize,
    eta_2: usize,
    d_u: usize,
    d_v: usize
}


impl KyberCPAPKE<512> {
    pub fn init() -> Self {
        Self {
            ring: PolynomialRing::new(KYBER_N_VALUE, KYBER_Q_VALUE),
            k: 2,
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
            ring: PolynomialRing::new(KYBER_N_VALUE, KYBER_Q_VALUE),
            k: 3,
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
            ring: PolynomialRing::new(KYBER_N_VALUE, KYBER_Q_VALUE),
            k: 4,
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
        let data = hash.split_at(KYBER_N_VALUE_IN_BYTES);
        (ByteArray::from(data.0), ByteArray::from(data.1))
    }

    fn xof(&self, bytes_arr: &ByteArray, first_byte: &ByteArray, second_byte: &ByteArray) -> Vec<u8> {
        let concat = ByteArray::concat(&[&bytes_arr, &first_byte, &second_byte]);
        shake_128(concat.get_bytes(), KYBER_XOF_DEFAULT_BYTES_STREAM_SIZE)
    }


    pub fn generate_matrix_from_seed(&self, seed: &ByteArray) -> PolynomialMatrix {
        let mut matrix_data = vec![];

        for i in 0..self.k{
            let mut row_data = vec![];

            for j in 0..self.k {
                let bytes_stream = self.xof(
                    seed,
                    &ByteArray::from(j),
                    &ByteArray::from(i)
                );
                let poly = self.ring.parse(ByteArray::from(bytes_stream.as_slice()));
                row_data.push(poly);
            }
            matrix_data.push(row_data);
        }

        matrix_data.into()
    }



    pub fn keygen(&self) -> (ByteArray, ByteArray) {
        let d = ByteArray::random(KYBER_N_VALUE_IN_BYTES);
        let (rho, sigma) = self.g(d);

        // Generating the A hat matrix
        let a_hat = self.generate_matrix_from_seed(&rho);


        (ByteArray::random(2), ByteArray::random(2))
    }
}

pub type KyberCPAPKE512 = KyberCPAPKE<512>;
pub type KyberCPAPKE768 = KyberCPAPKE<768>;
pub type KyberCPAPKE1024 = KyberCPAPKE<1024>;


#[cfg(test)]
mod tests {
    use crate::algorithms::kyber::constants::KYBER_N_VALUE_IN_BYTES;
    use crate::algorithms::kyber::cpapke::{KyberCPAPKE512};
    use crate::byte_array::ByteArray;

    #[test]
    fn test_g() {
        let seed = ByteArray::random(KYBER_N_VALUE_IN_BYTES);
        let kyber = KyberCPAPKE512::init();
        let (rho, sigma) = kyber.g(seed);
        assert_eq!(rho.get_bytes().len(), KYBER_N_VALUE_IN_BYTES);
        assert_eq!(sigma.get_bytes().len(), KYBER_N_VALUE_IN_BYTES);
    }

    #[test]
    fn test_generate_random_matrix_from_seed() {
        let seed = ByteArray::random(32);
        let kyber=  KyberCPAPKE512::init();
        let matrix = kyber.generate_matrix_from_seed(&seed);
        println!("{:?}", matrix)
    }
}