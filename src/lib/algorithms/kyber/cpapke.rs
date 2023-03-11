use crate::algebraic::galois_field::GaloisField;
use crate::algebraic::polynomial::Polynomial;
use crate::byte_array::ByteArray;
use crate::hash::{sha_512, shake_128, shake_256};
use crate::algorithms::kyber::constants::{KYBER_N_VALUE, KYBER_N_VALUE_IN_BYTES, KYBER_Q_VALUE, KYBER_XOF_DEFAULT_BYTES_STREAM_SIZE};
use crate::algorithms::kyber::galois_field::GF3329;
use crate::algorithms::kyber::matrix::MatrixRQ;
use crate::algorithms::kyber::polynomial::PolyRQ;

pub struct KyberCPAPKE<const V: usize> {
    k: u8,
    eta_1: u8,
    eta_2: u8,
    d_u: usize,
    d_v: usize
}


impl KyberCPAPKE<512> {
    pub fn init() -> Self {
        Self {
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

    /// This function corresponds to the Parse function mentioned in p.6 of the article.
    ///
    /// Input:
    ///     bytes_stream: An array of bytes
    /// Output:
    ///     A Polynomial belonging to the Polynomial Ring R_q
    fn parse(bytes_stream: ByteArray) -> PolyRQ {
        let mut coefficients = [GF3329::zero(); KYBER_N_VALUE];

        let mut i = 0;
        let mut j = 0;

        let bytes_arr = bytes_stream.get_bytes();

        while j  < KYBER_N_VALUE {
            let b_i = bytes_arr.get(i).unwrap().clone() as usize;
            let b_i_plus_one = bytes_arr.get(i+1).unwrap().clone() as usize;
            let b_i_plus_two = bytes_arr.get(i+2).unwrap().clone() as usize;

            let d_1 = b_i + KYBER_N_VALUE * (b_i_plus_one % 16 as usize);
            let d_2 = (b_i_plus_one / 16 as usize) + 16 as usize * b_i_plus_two;

            if d_1 < KYBER_Q_VALUE {
                coefficients[j] = d_1.into();
                j += 1;
            }

            if d_2 < KYBER_Q_VALUE && j < KYBER_N_VALUE {
                coefficients[j] = d_2.into();
                j += 1;
            }
            i += 3;
        }

        coefficients.into()
    }


    pub fn generate_matrix_from_seed(&self, seed: &ByteArray) -> MatrixRQ {
        let mut matrix_data = vec![];

        for i in 0..self.k{
            let mut row_data = vec![];

            for j in 0..self.k {
                let bytes_stream = self.xof(
                    seed,
                    j,
                    i
                );
                let poly = Self::parse(bytes_stream);
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

    fn cbd_eta(&self, bytes_array: &ByteArray, eta: u8) -> PolyRQ{
        // Checking length of bytes array
        if bytes_array.get_bytes().len() != (64 * eta) as usize {
            panic!("bytes_array does not have the correct size");
        }

        let mut coefficients = [GF3329::zero(); KYBER_N_VALUE];
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

            let diff: i32 = a as i32 - b as i32;
            coefficients[i] = diff.into();

        }

        coefficients.into()
    }

    fn generate_random_vec(&self, sigma: &ByteArray, upper_n: &mut u8, eta: u8) -> MatrixRQ {
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

        // // Generating s and e
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