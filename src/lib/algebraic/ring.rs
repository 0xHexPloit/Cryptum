use crate::algebraic::polynomial::Polynomial;
use crate::byte_array::ByteArray;

/// This structure permits to represent the ring  Zq[X]/(Xn+1)
#[derive(Debug, PartialEq)]
pub struct PolynomialRing {
    n: usize,
    q: usize
}

impl PolynomialRing {
    pub fn new(n: usize, q: usize) -> Self {
        Self {
            n,
            q
        }
    }

    pub fn get_characteristic(&self) ->  usize {
        self.q
    }

    pub fn get_order(&self) -> usize {
        self.n
    }

    /// This functions corresponds to the 'Parse' function as defined in the article (Algorithm 1)
    /// p.6
    ///
    /// Input:
    ///     - bytes_stream: A stream of bytes
    /// Output:
    ///     - An element of the ring
    pub fn parse(&self, bytes_stream: ByteArray) -> Polynomial {
        let mut i = 0;
        let mut j = 0;
        let mut coefficients: Vec<usize> = vec![0 as usize; self.n];

        let bytes_stream_bytes = bytes_stream.get_bytes();

        while j < self.n {
            let b_i = bytes_stream_bytes.get(i).unwrap().clone() as usize;
            let b_i_plus_one = bytes_stream_bytes.get(i+1).unwrap().clone() as usize;
            let b_i_plus_two = bytes_stream_bytes.get(i+2).unwrap().clone() as usize;

            let d_1 = b_i + 256 as usize * (b_i_plus_one % 16 as usize);
            let d_2 = (b_i_plus_one / 16 as usize) + 16 as usize * b_i_plus_two;

            if d_1 < self.q {
                coefficients[j] = d_1;
                j += 1;
            }
            if d_2 < self.q && j < self.n {
                coefficients[j] = d_2;
                j += 1;
            }

            i += 3;
        }

        Polynomial::new(&coefficients, &self)
    }
}