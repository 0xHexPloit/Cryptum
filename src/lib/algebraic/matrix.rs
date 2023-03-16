use crate::algebraic::polynomial::RingElement;

type Row<P> = Vec<P>;
type MatrixContent<P> = Vec<Row<P>>;

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

    pub fn get_row(&self, index: usize) -> &Row<P> {
        &self.data[index]
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


