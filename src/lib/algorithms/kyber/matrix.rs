// use crate::algebraic::matrix::Matrix;
// use crate::algorithms::kyber::ntt::NTT;
// use crate::algorithms::kyber::polynomial::PolyRQ;
//
// pub type MatrixRQ = Matrix<PolyRQ>;
//
// impl NTT for MatrixRQ {
//     fn inverse_ntt(self) -> Self {
//         let mut matrix_data = vec![];
//         let matrix_shape = self.get_shape();
//
//         for i in 0..matrix_shape.0 {
//             let mut row_data= vec![];
//
//             for j in 0..matrix_shape.1 {
//                 let poly = self.get_element(i, j);
//                 row_data.push(poly.inverse_ntt())
//             }
//             matrix_data.push(row_data)
//         }
//
//         matrix_data.into()
//     }
//
//     fn to_ntt(self) -> Self {
//         let mut matrix_data = vec![];
//         let matrix_shape = self.get_shape();
//
//         for i in 0..matrix_shape.0 {
//             let mut row_data= vec![];
//
//             for j in 0..matrix_shape.1 {
//                 let poly = self.get_element(i, j);
//                 row_data.push(poly.to_ntt())
//             }
//             matrix_data.push(row_data)
//         }
//
//         matrix_data.into()
//     }
//
//     fn ntt_multiply(&self, other: &Self) -> Self {
//         let mut row_data = vec![];
//
//
//         row_data.into()
//     }
// }