use std::ops::{Add};
use crate::algebraic::polynomial::RingElement;

type MatrixContent<P> = Vec<Vec<P>>;

fn check_matrix_content<P: RingElement>(content: &MatrixContent<P>) {
    // Checking that matrix contains some rows
    if content.len() == 0 {
        panic!("Matrix should content some rows")
    }

    let columns_numbers: Vec<usize> = content.iter().map(|row| row.len()).collect();
    let base_number = columns_numbers.get(0).unwrap();


    // Checking that all numbers are the same
    for i in 1..columns_numbers.len() {
        if *base_number != *columns_numbers.get(i).unwrap() {
            panic!("Some rows don't have the same number of columns")
        }
    }

    // Checking that base_number is not equal to 0
    if *base_number == 0 {
        panic!("A row should contain at least one element")
    }

}

#[derive(Debug)]
pub struct Matrix<P: RingElement> {
    data: MatrixContent<P>,
    number_rows: u8,
    number_columns: u8
}

impl <P: RingElement>From<MatrixContent<P>> for Matrix<P> {
    fn from(value: MatrixContent<P>) -> Self {
        check_matrix_content(&value);
        let number_rows = value.len() as u8;
        let number_columns = value.get(0).unwrap().len() as u8;

        Self {
            data: value,
            number_rows,
            number_columns
        }
    }
}


impl <P: RingElement> Matrix<P> {
    pub fn get_shape(&self) -> (u8, u8) {
        (self.number_rows, self.number_columns)
    }
    pub fn set(&mut self, i: u8, j: u8, content: P) {
        if i >= self.number_rows || j >= self.number_columns {
            panic!("Invalid coordinates!");
        }
        let mut_matrix_content: &mut MatrixContent<P> = self.data.as_mut();
        mut_matrix_content[i as usize][j as usize] = content;
    }

    pub fn get_element(&self, i: u8, j: u8) -> &P {
        if i >= self.number_rows || j >= self.number_columns {
            panic!("Invalid coordinates!");
        }
        &self.data[i as usize][j as usize]
    }
}

impl <P: RingElement>Add for Matrix<P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Checking that both matrices have the same shape
        let left_matrix_shape = self.get_shape();
        let right_matrix_shape = rhs.get_shape();

        if left_matrix_shape != right_matrix_shape {
            panic!("Cannot perform matrices addition ! They don't have the same shape!")
        }

        let mut matrix_data = Vec::with_capacity(left_matrix_shape.0 as usize);

        for i in 0..left_matrix_shape.0 {
            let mut row_data = Vec::with_capacity(left_matrix_shape.1 as usize);

            for j in 0..left_matrix_shape.1 {
                row_data.push(self.get_element(i, j).add(rhs.get_element(i, j)));
            }

            matrix_data.push(row_data);
        }

        matrix_data.into()
    }
}


#[cfg(test)]
mod tests {
    use crate::algebraic::galois_field::GaloisFieldCore;
    use crate::algebraic::matrix::Matrix;
    use crate::algebraic::polynomial::{Polynomial, RingElement};

    type GF7 = GaloisFieldCore<7>;
    type Poly7 = Polynomial<GF7, 2>;

    #[test]
    #[should_panic]
    fn test_matrix_creation_failed_empty_array() {
        let data = vec![];
        let _: Matrix<Poly7> = data.into();
    }

    #[test]
    #[should_panic]
    fn test_matrix_creation_failed_empty_array_first_row() {
        let data = vec![vec![]];
        let _: Matrix<Poly7> = data.into();
    }

    #[test]
    #[should_panic]
    fn test_matrix_creation_failed_columns_number_mismatch() {
        let poly_1 = Poly7::zero();
        let poly_2 = Poly7::zero();
        let poly_3 = Poly7::zero();

        let data = vec![
            vec![poly_1, poly_2],
            vec![poly_3]
        ];
        let _: Matrix<Poly7> = data.into();
    }

    #[test]
    fn test_matrix_creation_should_be_successful() {
        let poly_1 = Poly7::zero();
        let poly_2 = Poly7::zero();
        let poly_3 = Poly7::zero();
        let poly_4 = Poly7::zero();

        let data = vec![
            vec![poly_1, poly_2],
            vec![poly_3, poly_4]
        ];
        let matrix: Matrix<Poly7> = data.into();

        assert_eq!(matrix.number_rows, 2);
        assert_eq!(matrix.number_columns, 2)
    }
}


