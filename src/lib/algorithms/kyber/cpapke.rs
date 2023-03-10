use std::iter::Map;
use crate::algebraic::matrix::PolynomialMatrix;
use crate::algebraic::polynomial::Polynomial;
use crate::byte_array::ByteArray;
use crate::hash::{sha_512, shake_128, shake_256};
use crate::algebraic::ring::PolynomialRing;
use crate::algorithms::kyber::constants::{KYBER_N_VALUE, KYBER_N_VALUE_IN_BYTES, KYBER_Q_VALUE, KYBER_XOF_DEFAULT_BYTES_STREAM_SIZE};

pub struct KyberCPAPKE<const V: usize> {
    ring: PolynomialRing,
    k: u8,
    eta_1: u8,
    eta_2: u8,
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

    /// This function corresponds to the XOF function as defined in p5 of the article.
    ///
    /// Input:
    ///     bytes_arr: An array containing 32 bytes
    ///     first_byte: A byte value
    ///     second_byte: A byte value
    /// Output:
    ///     A stream of bytes
    fn xof(&self, bytes_arr: &ByteArray, first_byte: u8, second_byte: u8) -> ByteArray {
        let concat = ByteArray::concat(&[
            &bytes_arr,
            &first_byte.into(),
            &second_byte.into()
        ]);
        shake_128(concat.get_bytes(), KYBER_XOF_DEFAULT_BYTES_STREAM_SIZE).into()
    }


    pub fn generate_matrix_from_seed(&self, seed: &ByteArray) -> PolynomialMatrix {
        let mut matrix_data = vec![];

        for i in 0..self.k{
            let mut row_data = vec![];

            for j in 0..self.k {
                let bytes_stream = self.xof(
                    seed,
                    j,
                    i
                );
                let poly = self.ring.parse(ByteArray::from(bytes_stream.get_bytes()));
                row_data.push(poly);
            }
            matrix_data.push(row_data);
        }

        matrix_data.into()
    }


    /// This function corresponds to the PRF function as defined in p5 of the article.
    ///
    /// Input:
    ///     s: An array containing 32 bytes
    ///     b: A byte value
    ///     length: The length of the returned bytes stream
    /// Output:
    ///     A stream of bytes
    fn prf(&self, s: &ByteArray, b: u8, length: usize) -> ByteArray {
        let data = ByteArray::concat(&[&s, &b.into()]);

        // Checking length of data
        if data.get_bytes().len() != 33 {
            panic!("The array of bytes obtained by concatenating s and b is not equal to 32!")
        }
        shake_256(data.get_bytes(), length).into()

    }

    fn cbd_eta(&self, bytes_array: &ByteArray, eta: u8) -> Polynomial{
        // Checking length of bytes array
        if bytes_array.get_bytes().len() != (64 * eta) as usize {
            panic!("bytes_array does not have the correct size");
        }

        let mut coefficients: Vec<usize> = Vec::with_capacity(KYBER_N_VALUE);
        let bits = bytes_array.to_bits();

        for i in 0..256 {
            let mut a = 0 as usize;
            let mut b = 0 as usize;

            for j in 0..eta {
                let a_index = 2 * i  * eta as usize + j as usize;
                a += bits[a_index] as usize;

                let b_index = 2 * i * eta as usize + eta as usize + j as usize;
                b += bits[b_index] as usize;
            }

            let coefficient: isize = (a as isize - b as isize);


            coefficients.push(coefficient as usize % self.ring.get_characteristic());
        }

        Polynomial::new(&coefficients, &self.ring)
    }

    fn generate_random_vec(&self, sigma: &ByteArray, upper_n: &mut u8, eta: u8) -> PolynomialMatrix {
        let mut matrix_data = Vec::with_capacity(self.k as usize);

        for i in 0..self.k {
            let bytes_arr = self.prf(sigma, *upper_n, 64 * self.eta_1 as usize);
            let poly = self.cbd_eta(&bytes_arr, eta);
            matrix_data.push(vec![poly]);
            *upper_n += 1;
        }

        matrix_data.into()
    }



    pub fn keygen(&self) -> (ByteArray, ByteArray) {
        let d = ByteArray::random(KYBER_N_VALUE_IN_BYTES);
        let (rho, sigma) = self.g(d);

        // Generating the A hat matrix
        let a_hat = self.generate_matrix_from_seed(&rho);

        // Generating s and e
        let mut upper_n = 0;
        let s = self.generate_random_vec(&sigma, &mut upper_n, self.eta_1);
        let e = self.generate_random_vec(&sigma, &mut upper_n, self.eta_1);



        (ByteArray::random(2), ByteArray::random(2))
    }
}

pub type KyberCPAPKE512 = KyberCPAPKE<512>;
pub type KyberCPAPKE768 = KyberCPAPKE<768>;
pub type KyberCPAPKE1024 = KyberCPAPKE<1024>;


#[cfg(test)]
mod tests {
    use hex_literal::hex;
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

    #[test]
    fn test_prf_func() {
        let bytes_array = ByteArray::from([0x41; 32].as_slice());
        let expected_size = 10;
        let kyber=  KyberCPAPKE512::init();
        let output = kyber.prf(&bytes_array, 0x42, expected_size);

        assert_eq!(output.get_bytes().len(), expected_size);
        assert_eq!(output.get_bytes(), hex!("1aef8fd492d01f8e69a3"))
    }

    #[test]
    #[should_panic]
    fn test_prf_panic_if_data_length_not_equal_to_33() {
        let bytes_array = ByteArray::from([0x41; 2].as_slice());
        let expected_size = 10;
        let kyber=  KyberCPAPKE512::init();
        let _ = kyber.prf(&bytes_array, 0x42, expected_size);
    }

    #[test]
    fn test_keygen() {
        let seed = ByteArray::random(32);
        let kyber=  KyberCPAPKE512::init();
        let _ = kyber.keygen();
    }
}