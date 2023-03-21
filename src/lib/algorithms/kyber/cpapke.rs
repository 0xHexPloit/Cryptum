use crate::algorithms::algebraic::galois_field::GaloisField;
use crate::algorithms::algebraic::polynomial::RingElement;
use crate::algorithms::byte_array::ByteArray;
use crate::algorithms::kyber::compress::{Compress, Decompress};
use crate::algorithms::kyber::constants::{KYBER_MESSAGE_LENGTH, KYBER_N_VALUE, KYBER_N_VALUE_IN_BYTES, KYBER_Q_VALUE, KYBER_RANDOM_COIN_LENGTH, KYBER_XOF_DEFAULT_BYTES_STREAM_SIZE};
use crate::algorithms::kyber::encoder::{Decoder, Encoder};
use crate::algorithms::kyber::galois_field::GF3329;
use crate::algorithms::kyber::matrix::MatrixRQ;
use crate::algorithms::kyber::ntt::NTT;

use crate::algorithms::kyber::polynomial::PolyRQ;
use crate::algorithms::kyber::vector::VectorRQ;
use crate::algorithms::utils::hash::{sha3_512, shake_128, shake_256};

pub trait KyberPKE {
    fn keygen(&self, seed: ByteArray) -> (ByteArray, ByteArray);
    fn encrypt(&self, public_key: ByteArray, message: ByteArray, random_coin: ByteArray) -> ByteArray;
    fn decrypt(&self, private_key: ByteArray, ciphertext: ByteArray) -> ByteArray;
}


pub struct KyberCPAPKECore<const V: usize> {
    k: u8,
    eta_1: u8,
    eta_2: u8,
    d_u: usize,
    d_v: usize
}


impl KyberCPAPKECore<512> {
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

impl KyberCPAPKECore<768> {
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

impl KyberCPAPKECore<1024> {
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


impl <const N: usize> KyberCPAPKECore<N> {
    pub fn get_k(&self) -> u8 {
        self.k
    }

    pub fn get_public_key_length(&self) -> usize {
        (12 * self.k as usize * KYBER_N_VALUE / 8) + 32
    }

    fn g(&self, seed: ByteArray) -> (ByteArray, ByteArray) {
        let hash = sha3_512(seed.get_bytes());
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

        for i in 0..KYBER_N_VALUE {
            let mut a = GF3329::zero();
            let mut b = GF3329::zero();

            for j in 0..eta {
                let a_index = 2 * i  * eta as usize + j as usize;
                a = a.add(&GF3329::from(bits[a_index] as usize));

                let b_index = 2 * i * eta as usize + eta as usize + j as usize;
                b = b.add(&GF3329::from(bits[b_index] as usize));
            }

            coefficients[i] = a.sub(&b);

        }

        coefficients.into()
    }


    fn generate_random_poly(&self, sigma: &ByteArray, upper_n: u8, eta: u8) -> PolyRQ {
        let bytes_arr = self.prf(sigma, upper_n, 64 * eta as usize);
        self.cbd_eta(&bytes_arr, eta)
    }

    fn generate_random_vec(&self, sigma: &ByteArray, upper_n: &mut u8, eta: u8) -> VectorRQ {
        let mut vector_data = Vec::with_capacity(self.k as usize);

        for _ in 0..self.k {
            let poly = self.generate_random_poly(sigma, *upper_n, eta);
            vector_data.push(poly);
            *upper_n += 1;
        }

        vector_data.into()
    }

    fn decode_vec(&self, public_key: &ByteArray, l_value: u8) -> VectorRQ {
        let mut polynomials = vec![];
        let bytes = public_key.get_bytes();

        for chunk in bytes.chunks_exact(32 * l_value as usize) {
            let sub_bytes_array = ByteArray::from(chunk);
            let polynomial = PolyRQ::decode(sub_bytes_array, l_value);
            polynomials.push(polynomial);
        }

        polynomials.into()
    }

    pub fn get_private_key_length(&self) -> usize {
        (12 * self.k as usize * KYBER_N_VALUE) / 8
    }

    pub fn get_ciphertext_length(&self) -> usize {
        (self.d_u * self.k as usize * KYBER_N_VALUE) / 8 + (self.d_v * KYBER_N_VALUE) / 8
    }
}

impl <const V: usize>KyberPKE for KyberCPAPKECore<V> {
    /// This function corresponds to the KeyGen function (Algorithm 4). We only modify it to take
    /// a seed as an input to perform some unit test. In the paper, the seed is computed inside
    /// the function.
    ///
    /// Input:
    ///     seed: A 32-bytes array
    /// Output:
    ///     - An array of bytes corresponding to the public key
    ///     - An array of bytes corresponding to the private key
    fn keygen(&self, seed: ByteArray) -> (ByteArray, ByteArray) {
        // Checking length of seed
        if seed.length() != 32 {
            panic!("Invalid for seed ! Should be 32 found {}", seed.length());
        }

        let (rho, sigma) = self.g(seed);

        // Generating the A hat matrix
        let a_hat = self.generate_matrix_from_seed(&rho);

        // // Generating s and e
        let mut upper_n = 0;
        let s = self.generate_random_vec(&sigma, &mut upper_n, self.eta_1);
        let e = self.generate_random_vec(&sigma, &mut upper_n, self.eta_1);

        // Applying NTT transformation to s and e
        let s_hat = s.to_ntt();
        let e_hat = e.to_ntt();

        let t_hat = a_hat.multiply_vec(&s_hat) + e_hat;

        let public_key = ByteArray::concat(&[&t_hat.encode(12), &rho]);
        let private_key = s_hat.encode(12);


        (public_key, private_key)
    }

    /// This function corresponds to the Enc function (Algorithm 5).
    ///
    /// Input:
    ///     public_key: An array of bytes that corresponds to the public key
    ///     message: A 32-bytes array to cipher
    ///     random_coin: An array of bytes filled with random values
    /// Output:
    ///     An array of bytes corresponding to a ciphertext
    fn encrypt(&self, public_key: ByteArray, message: ByteArray, random_coin: ByteArray) -> ByteArray {
        // Checking the length of the public key
        let public_key_length = public_key.length();

        let expected_public_key_length = self.get_public_key_length();
        if public_key_length !=  expected_public_key_length {
            panic!(
                "Invalid length for public key ! Expected {} found {}",
                expected_public_key_length,
                public_key.length()
            )
        }

        // Checking length for message
        let message_length = message.length();

        if message_length != KYBER_MESSAGE_LENGTH {
            panic!("Invalid length for message ! Expected 32 found {}.", message_length);
        }

        // Checking length for random coin
        let random_coin_length = random_coin.length();

        if random_coin_length != KYBER_RANDOM_COIN_LENGTH {
            panic!("Invalid length for random coin ! Expected 32 found {}.", random_coin_length);
        }

        let mut upper_n = 0;

        let t_hat = self.decode_vec(&public_key, 12);
        let rho = public_key.slice(public_key_length - 32);

        let a_hat = self.generate_matrix_from_seed(&rho);

        let r = self.generate_random_vec(&random_coin, &mut upper_n, self.eta_1);
        let e_1 = self.generate_random_vec(&random_coin, &mut upper_n, self.eta_2);
        let e_2 = self.generate_random_poly(&random_coin, upper_n, self.eta_2);

        let r_hat = r.to_ntt();

        let u: VectorRQ = (a_hat.transpose().multiply_vec(&r_hat)).inverse_ntt() + e_1;

        let m = PolyRQ::decode(message, 1).decompress(1);

        let v = (t_hat.dot_ntt(&r_hat))
            .inverse_ntt()
            .add(&e_2)
            .add(&m);


        let c_1 = u.compress(self.d_u as u32).encode(self.d_u);
        let c_2 = v.compress(self.d_v as u32).encode(self.d_v);

        ByteArray::concat(&[&c_1, &c_2])
    }

    /// This function corresponds to the Dec function (Algorithm 6).
    ///
    /// Input:
    ///     private_key: An array of bytes that corresponds to a private key generate by the KeyGen function.
    ///     ciphertext: An array of bytes to decipher.
    /// Output:
    ///     An array of bytes that represents the original message that has been ciphered previously.
    fn decrypt(&self, private_key: ByteArray, ciphertext: ByteArray) -> ByteArray {
        // Checking length of private
        let private_key_length = private_key.length();
        let expected_private_key_length = self.get_private_key_length();

        if private_key_length !=  expected_private_key_length {
            panic!(
                "Invalid length for secret_key ! Expected {} found {}",
                expected_private_key_length,
                private_key_length
            )
        }

        // Checking length of ciphertext
        let ciphertext_length = ciphertext.length();

        if ciphertext_length != self.get_ciphertext_length() {
            panic!(
                "Invalid length for ciphertext ! Expected {} found {}",
                self.get_ciphertext_length(),
                ciphertext_length
            );
        }

        let split_pos = (self.d_u * self.k as usize * KYBER_N_VALUE) / 8;
        let (c_1, c_2) = ciphertext.split_at(split_pos);


        let u = VectorRQ::decode(c_1, self.d_u as u8).decompress(self.d_u as u32);
        let v = PolyRQ::decode(c_2, self.d_v as u8).decompress(self.d_v as u32);
        let s_hat = VectorRQ::decode(private_key, 12);

        let mut poly = (s_hat.dot_ntt(&u.to_ntt())).inverse_ntt();
        poly = v.sub(&poly);

        poly.compress(1).encode(1)
    }
}


pub type KyberCPAPKE512 = KyberCPAPKECore<512>;
pub type KyberCPAPKE768 = KyberCPAPKECore<768>;
pub type KyberCPAPKE1024 = KyberCPAPKECore<1024>;


#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use crate::algorithms::byte_array::ByteArray;
    use crate::algorithms::kyber::constants::{KYBER_MESSAGE_LENGTH, KYBER_N_VALUE, KYBER_N_VALUE_IN_BYTES, KYBER_RANDOM_COIN_LENGTH};
    use crate::algorithms::kyber::cpapke::{KyberCPAPKE512};
    use crate::algorithms::kyber::KyberPKE;

    #[test]
    fn test_g() {
        let seed = ByteArray::from([0u8; KYBER_N_VALUE_IN_BYTES].as_slice());
        let kyber = KyberCPAPKE512::init();
        let (rho, sigma) = kyber.g(seed);

        let expected_rho = [173, 86, 195, 92, 171, 80, 99, 185, 231, 234, 86, 131, 20, 236, 129, 196, 11, 165, 119, 170, 230, 48, 222, 144, 32, 4, 0, 158, 136, 241, 141, 165];
        let expected_rho = ByteArray::from(expected_rho.as_slice());

        let expected_sigma = [123, 189, 253, 170, 160, 252, 24, 156, 102, 200, 216, 83, 36, 139, 107, 17, 136, 68, 213, 63, 125, 11, 161, 29, 224, 243, 191, 175, 76, 221, 155, 63];
        let expected_sigma = ByteArray::from(expected_sigma.as_slice());

        assert_eq!(rho, expected_rho);
        assert_eq!(sigma, expected_sigma);
    }

    #[test]
    fn test_generate_random_matrix_from_seed() {
        let seed = ByteArray::random(32);
        let kyber=  KyberCPAPKE512::init();
        let _ = kyber.generate_matrix_from_seed(&seed);
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
    fn test_keygen_length() {
        let kyber=  KyberCPAPKE512::init();
        let seed = ByteArray::random(32);
        let (public_key, private_key) = kyber.keygen(seed);

        let expected_length_public_key = 800;
        let expected_length_private_key = 768;

        assert_eq!(public_key.length(), expected_length_public_key);
        assert_eq!(private_key.length(), expected_length_private_key)

    }

    #[test]
    fn test_keygen() {
        let kyber = KyberCPAPKE512::init();
        let seed = ByteArray::from([0u8; KYBER_N_VALUE_IN_BYTES].as_slice());
        let (public_key, private_key) = kyber.keygen(seed);

        let expected_public_key = [156, 229, 198, 126, 252, 156, 51, 103, 168, 60, 124, 66, 176, 161, 41, 55, 201, 64, 207, 113, 26, 162, 167, 127, 148, 56, 146, 141, 105, 80, 234, 96, 7, 192, 185, 2, 255, 25, 119, 123, 32, 147, 18, 102, 6, 119, 0, 185, 105, 160, 36, 146, 236, 192, 102, 115, 206, 246, 225, 206, 54, 119, 58, 75, 219, 149, 56, 213, 52, 88, 38, 24, 222, 210, 121, 226, 153, 161, 32, 35, 24, 19, 140, 168, 213, 73, 182, 198, 8, 129, 238, 210, 183, 74, 184, 132, 185, 48, 116, 104, 83, 89, 96, 53, 39, 236, 99, 139, 171, 177, 61, 152, 242, 167, 62, 42, 54, 42, 86, 98, 163, 81, 83, 141, 83, 87, 236, 124, 32, 182, 103, 64, 128, 199, 106, 83, 101, 171, 159, 72, 63, 120, 108, 78, 44, 219, 202, 35, 240, 170, 10, 100, 88, 158, 184, 110, 170, 7, 55, 113, 240, 77, 57, 128, 142, 144, 68, 55, 104, 170, 13, 9, 34, 181, 223, 161, 84, 231, 231, 9, 61, 52, 34, 56, 20, 186, 161, 182, 109, 174, 54, 83, 206, 162, 21, 179, 172, 125, 108, 101, 184, 44, 49, 174, 223, 192, 62, 62, 246, 29, 210, 147, 21, 181, 193, 28, 16, 165, 185, 223, 19, 146, 23, 160, 194, 62, 244, 191, 129, 65, 14, 191, 22, 15, 91, 186, 77, 60, 103, 24, 179, 86, 177, 133, 101, 90, 190, 209, 66, 163, 196, 153, 5, 226, 40, 157, 212, 202, 5, 160, 58, 5, 11, 160, 8, 211, 82, 189, 34, 186, 235, 137, 120, 78, 252, 41, 1, 12, 162, 120, 74, 108, 33, 203, 70, 147, 121, 0, 153, 219, 158, 202, 147, 37, 81, 105, 149, 49, 100, 21, 136, 140, 53, 168, 52, 97, 187, 90, 134, 180, 37, 23, 178, 177, 86, 210, 133, 152, 201, 91, 117, 78, 27, 55, 79, 64, 160, 174, 83, 33, 105, 114, 118, 136, 103, 205, 69, 167, 202, 214, 213, 167, 56, 43, 42, 93, 124, 22, 148, 73, 110, 5, 72, 173, 244, 87, 27, 75, 92, 95, 93, 33, 147, 159, 188, 155, 242, 105, 205, 175, 83, 190, 95, 9, 190, 97, 147, 174, 186, 89, 31, 79, 193, 185, 67, 151, 121, 141, 185, 66, 231, 58, 117, 89, 0, 23, 190, 75, 70, 193, 233, 77, 208, 251, 27, 138, 148, 118, 242, 138, 207, 11, 33, 26, 193, 202, 128, 44, 168, 114, 62, 122, 111, 97, 241, 52, 60, 23, 69, 158, 85, 178, 115, 44, 206, 246, 60, 128, 190, 67, 89, 247, 246, 21, 249, 12, 46, 140, 108, 8, 35, 133, 174, 183, 21, 24, 137, 42, 28, 135, 203, 63, 54, 102, 129, 71, 217, 25, 193, 6, 107, 170, 105, 182, 227, 249, 153, 249, 38, 120, 13, 102, 23, 112, 21, 35, 64, 115, 29, 2, 118, 196, 84, 168, 108, 130, 224, 183, 76, 75, 76, 27, 119, 55, 242, 249, 174, 50, 34, 183, 208, 242, 203, 192, 1, 186, 84, 33, 204, 106, 26, 34, 208, 133, 135, 162, 88, 206, 66, 188, 105, 161, 114, 68, 118, 165, 167, 150, 185, 127, 161, 92, 133, 156, 124, 96, 106, 227, 44, 129, 188, 153, 57, 105, 153, 211, 69, 123, 180, 35, 149, 99, 42, 182, 153, 83, 64, 181, 20, 128, 15, 88, 46, 86, 149, 52, 208, 26, 39, 66, 220, 47, 10, 53, 191, 173, 97, 134, 56, 181, 155, 207, 195, 59, 180, 194, 77, 53, 231, 101, 98, 252, 75, 220, 68, 180, 22, 65, 49, 59, 12, 95, 202, 146, 108, 84, 211, 198, 66, 229, 42, 98, 6, 56, 103, 200, 186, 58, 10, 121, 118, 19, 94, 254, 59, 108, 73, 234, 38, 93, 85, 162, 209, 213, 103, 127, 73, 176, 158, 186, 56, 35, 74, 190, 56, 179, 149, 39, 168, 23, 66, 20, 78, 147, 67, 130, 178, 117, 100, 41, 42, 36, 214, 165, 56, 145, 107, 33, 72, 210, 63, 120, 231, 10, 89, 233, 128, 150, 201, 18, 3, 247, 121, 101, 33, 31, 46, 234, 47, 198, 252, 31, 89, 99, 78, 36, 107, 12, 158, 36, 118, 156, 229, 169, 79, 194, 124, 51, 213, 86, 80, 2, 143, 149, 27, 42, 48, 117, 201, 139, 135, 135, 70, 224, 61, 201, 28, 191, 221, 151, 37, 236, 9, 180, 77, 185, 15, 12, 102, 43, 165, 9, 110, 157, 171, 7, 106, 123, 123, 26, 202, 180, 235, 43, 121, 91, 101, 49, 173, 86, 195, 92, 171, 80, 99, 185, 231, 234, 86, 131, 20, 236, 129, 196, 11, 165, 119, 170, 230, 48, 222, 144, 32, 4, 0, 158, 136, 241, 141, 165];
        let expected_public_key = ByteArray::from(expected_public_key.as_slice());

        let expected_private_key = [50, 193, 136, 186, 115, 194, 8, 3, 37, 219, 64, 59, 240, 247, 22, 1, 2, 119, 108, 71, 17, 52, 10, 26, 180, 134, 47, 42, 16, 47, 205, 74, 161, 57, 199, 207, 249, 241, 190, 196, 51, 68, 1, 144, 110, 56, 132, 70, 62, 148, 35, 15, 219, 28, 211, 183, 119, 38, 51, 97, 60, 73, 178, 2, 0, 166, 158, 48, 96, 52, 33, 88, 84, 103, 112, 21, 49, 74, 233, 213, 147, 139, 169, 184, 167, 129, 175, 49, 129, 8, 146, 89, 84, 61, 133, 172, 80, 55, 137, 13, 51, 67, 1, 231, 117, 236, 99, 29, 59, 69, 111, 20, 7, 2, 110, 116, 190, 142, 119, 64, 124, 153, 203, 198, 88, 163, 93, 59, 23, 177, 54, 33, 249, 38, 98, 185, 96, 180, 13, 27, 193, 74, 35, 72, 140, 248, 163, 228, 121, 200, 55, 98, 86, 234, 196, 10, 237, 69, 67, 104, 90, 97, 43, 135, 123, 166, 114, 186, 22, 188, 63, 208, 169, 77, 137, 226, 201, 51, 248, 35, 161, 169, 105, 159, 247, 182, 151, 155, 109, 45, 51, 142, 200, 50, 147, 234, 73, 150, 91, 2, 86, 101, 153, 27, 76, 7, 169, 109, 25, 86, 43, 179, 146, 234, 188, 168, 68, 114, 124, 127, 233, 87, 219, 151, 176, 132, 9, 185, 255, 184, 205, 206, 87, 68, 199, 220, 190, 199, 228, 98, 186, 150, 174, 244, 183, 169, 198, 42, 174, 88, 7, 188, 229, 219, 91, 16, 118, 148, 56, 120, 64, 80, 52, 25, 26, 75, 85, 25, 161, 193, 139, 162, 153, 26, 168, 73, 100, 22, 165, 171, 240, 65, 221, 120, 78, 39, 249, 158, 221, 155, 196, 108, 70, 78, 104, 200, 21, 115, 224, 155, 133, 228, 87, 100, 48, 185, 86, 81, 107, 4, 128, 21, 102, 58, 187, 91, 8, 8, 66, 52, 144, 204, 26, 13, 75, 97, 109, 17, 138, 172, 50, 199, 188, 119, 129, 103, 78, 152, 108, 153, 167, 62, 61, 234, 206, 77, 21, 26, 21, 36, 186, 106, 101, 21, 124, 130, 104, 190, 42, 13, 28, 245, 186, 0, 145, 102, 179, 41, 25, 46, 156, 137, 25, 83, 62, 192, 124, 78, 85, 122, 25, 94, 231, 85, 190, 71, 145, 39, 133, 109, 156, 74, 20, 58, 168, 7, 183, 106, 130, 166, 249, 168, 15, 81, 189, 93, 150, 185, 26, 32, 148, 228, 48, 193, 35, 6, 19, 56, 209, 5, 166, 1, 118, 56, 10, 113, 5, 107, 170, 194, 204, 18, 217, 8, 183, 149, 172, 149, 255, 203, 127, 30, 72, 9, 63, 129, 10, 250, 16, 59, 177, 10, 17, 59, 52, 58, 152, 235, 102, 152, 112, 9, 228, 2, 87, 187, 197, 107, 128, 227, 192, 107, 120, 42, 138, 196, 112, 153, 102, 184, 159, 17, 101, 52, 73, 60, 153, 11, 136, 0, 16, 9, 189, 213, 125, 3, 137, 162, 215, 119, 163, 63, 28, 195, 8, 108, 38, 200, 217, 113, 230, 54, 147, 60, 99, 86, 220, 248, 187, 8, 35, 2, 90, 153, 81, 193, 241, 58, 11, 10, 89, 103, 209, 180, 4, 180, 154, 44, 122, 167, 253, 12, 76, 49, 19, 138, 60, 107, 83, 73, 4, 110, 47, 122, 68, 140, 53, 0, 207, 92, 179, 84, 135, 54, 113, 96, 168, 73, 247, 153, 83, 0, 131, 156, 114, 28, 214, 91, 170, 47, 115, 128, 182, 44, 38, 180, 197, 143, 35, 122, 81, 73, 185, 186, 125, 89, 82, 206, 37, 105, 111, 4, 185, 208, 150, 29, 21, 136, 27, 72, 134, 201, 232, 198, 162, 238, 35, 8, 49, 12, 188, 167, 3, 82, 8, 83, 199, 189, 120, 90, 76, 120, 62, 194, 251, 198, 220, 25, 95, 100, 22, 84, 16, 243, 54, 107, 99, 104, 0, 199, 203, 94, 228, 29, 47, 188, 105, 91, 2, 117, 250, 121, 140, 53, 22, 103, 32, 90, 129, 5, 81, 35, 175, 122, 148, 39, 236, 17, 9, 251, 95, 78, 59, 35, 195, 106, 21, 246, 64, 69, 178, 80, 123, 36, 24, 116, 78, 99, 8, 49, 183, 192, 251, 212, 191, 77, 67, 79, 41, 215, 127, 84, 212, 57, 114, 59, 50, 2, 185, 154, 74, 226, 170, 91, 26, 167, 146, 193, 44, 87, 7, 112, 176, 101, 22, 150, 234, 60, 190, 251, 6, 200, 52, 141, 95, 37, 133, 47, 251, 51, 181, 214, 83, 197, 41, 92, 153, 144, 115, 168, 36, 102, 218, 194, 48, 122, 200, 23];
        let expected_private_key = ByteArray::from(expected_private_key.as_slice());

        assert_eq!(public_key, expected_public_key);
        assert_eq!(private_key, expected_private_key);
    }

    #[test]
    #[should_panic]
    fn test_failed_encryption_invalid_length_for_public_key() {
        let public_key = ByteArray::random(2);
        let message = ByteArray::random(2);
        let random_coin = ByteArray::random(2);

        let kyber = KyberCPAPKE512::init();
        let _ = kyber.encrypt(public_key, message, random_coin);
    }

    #[test]
    #[should_panic]
    fn test_failed_encrypt_invalid_length_for_message() {
        let kyber = KyberCPAPKE512::init();
        let public_key_length = kyber.get_public_key_length();
        let public_key = ByteArray::random(public_key_length);
        let message = ByteArray::random(2);
        let random_coin = ByteArray::random(2);

        let _ = kyber.encrypt(public_key, message, random_coin);
    }

    #[test]
    #[should_panic]
    fn test_failed_encrypt_invalid_length_for_random_coin() {
        let kyber = KyberCPAPKE512::init();
        let public_key_length = kyber.get_public_key_length();
        let public_key = ByteArray::random(public_key_length);
        let message = ByteArray::random(KYBER_MESSAGE_LENGTH);
        let random_coin = ByteArray::random(2);

        let _ = kyber.encrypt(public_key, message, random_coin);
    }

    #[test]
    fn test_encrypt_length() {
        let kyber = KyberCPAPKE512::init();
        let public_key_length = kyber.get_public_key_length();
        let public_key = ByteArray::random(public_key_length);
        let message = ByteArray::random(KYBER_MESSAGE_LENGTH);
        let random_coin = ByteArray::random(KYBER_RANDOM_COIN_LENGTH);

        let expected_length = kyber.get_ciphertext_length();

        let encryption = kyber.encrypt(public_key, message, random_coin);

        assert_eq!(encryption.length(), expected_length);
    }

    #[test]
    #[should_panic]
    fn test_decrypt_should_panic_invalid_length_secret_key() {
        let secret_key = ByteArray::random(2);
        let ciphertext = ByteArray::random(2);

        let kyber = KyberCPAPKE512::init();
        let _ = kyber.decrypt(secret_key, ciphertext);

    }

    #[test]
    #[should_panic]
    fn test_decrypt_should_panic_invalid_length_ciphertext() {
        let kyber = KyberCPAPKE512::init();
        let secret_key_length = kyber.get_private_key_length();
        let secret_key = ByteArray::random(secret_key_length);
        let ciphertext = ByteArray::random(2);

        let _ = kyber.decrypt(secret_key, ciphertext);
    }

    #[test]
    fn test_decrypt_length() {
        let kyber = KyberCPAPKE512::init();
        let secret_key = ByteArray::random(kyber.get_private_key_length());

        let ciphertext = ByteArray::random(kyber.get_ciphertext_length());

        let output = kyber.decrypt(secret_key, ciphertext);

        assert_eq!(output.length(), KYBER_MESSAGE_LENGTH)
    }

    #[test]
    fn test_encrypt_decrypt() {
        let kyber = KyberCPAPKE512::init();
        let seed = [0_u8; KYBER_N_VALUE_IN_BYTES];

        let (public_key, private_key) = kyber.keygen(seed.as_slice().into());

        let random_coin = [0_u8; KYBER_RANDOM_COIN_LENGTH];

        let original_message = "Telecom PARIS".as_bytes();
        let mut message_data = [0u8; 32];
        message_data[0..original_message.len()].copy_from_slice(original_message);
        let message_bytes: ByteArray = message_data.as_slice().into();

        let ciphertext = kyber.encrypt(public_key, message_bytes.clone(), random_coin.as_slice().into());
        let plaintext = kyber.decrypt(private_key, ciphertext);

        assert_eq!(plaintext, message_bytes)
    }
}