use crate::algebraic::polynomial::Polynomial;

type MatrixData<'a> = Vec<Vec<Polynomial<'a>>>;


fn check_matrix_content(data: &MatrixData) {
    // Checking that the matrix contains some rows
    if data.len() == 0 {
        panic!("The matrix should contain some rows");
    }

    let columns_first_row = data.get(0).unwrap();

    // Checking that the matrix contains some columns
    if columns_first_row.len() == 0 {
        panic!("The matrix should contain some columns");
    }

    // Checking matrix consistency in terms of number of columns per row
    if data.len() > 1 {
        for i in 1..data.len() {
            if columns_first_row.len() != data.get(i).unwrap().len() {
                panic!("Different number of columns detected!")
            }
        }
    }

    // Checking that all the polynomials belong to the same ring
    let base_ring_ref = columns_first_row.get(0).unwrap().get_ring_ref();
    for i in 0..data.len() {
        for j in 0..columns_first_row.len() {
            let row = data.get(i).unwrap();
            let current_poly = row.get(j).unwrap();

            if base_ring_ref != current_poly.get_ring_ref() {
                panic!("all the elements of the matrix should belong to the same ring");
            }
        }
    }
}


#[derive(Debug)]
pub struct PolynomialMatrix<'a> {
    data: MatrixData<'a>,
    num_rows: u8,
    num_columns: u8
}

impl <'a>From<MatrixData<'a>> for PolynomialMatrix<'a> {
    fn from(value: MatrixData<'a>) -> Self {
        check_matrix_content(&value);

        let num_rows = value.len() as u8;
        let num_columns = value.get(0).unwrap().len() as u8;
        Self {
            data: value,
            num_rows,
            num_columns
        }
    }
}


impl <'a>PolynomialMatrix<'a> {
    pub fn get_shape(&self) -> (u8, u8) {
        (self.num_rows, self.num_columns)
    }
}

#[cfg(test)]
mod tests {
    use crate::algebraic::matrix::{MatrixData, PolynomialMatrix};
    use crate::algebraic::polynomial::Polynomial;
    use crate::algebraic::ring::PolynomialRing;

    #[test]
    #[should_panic]
    fn test_failed_matrix_creation_empty_rows() {
        let data: MatrixData = vec![];
        let _: PolynomialMatrix = data.into();
    }

    #[test]
    #[should_panic]
    fn test_failed_matrix_creation_empty_columns() {
        let data: MatrixData = vec![vec![]];
        let _: PolynomialMatrix = data.into();
    }

    #[test]
    #[should_panic]
    fn test_failed_matrix_creation_inconsistency_in_terms_of_number_of_columns() {
        let ring = PolynomialRing::new(3, 5);
        let poly_1 = Polynomial::zero(&ring);
        let poly_2 = Polynomial::zero(&ring);
        let poly_3 = Polynomial::zero(&ring);


        let data: MatrixData = vec![vec![poly_1, poly_2], vec![poly_3]];
        let _: PolynomialMatrix = data.into();
    }

    #[test]
    #[should_panic]
    fn test_failed_matrix_creation_not_same_ring_ref() {
        let ring_1 = PolynomialRing::new(3, 5);
        let ring_2 = PolynomialRing::new(3, 7);
        let poly_1 = Polynomial::zero(&ring_1);
        let poly_2 = Polynomial::zero(&ring_2);

        let data: MatrixData = vec![vec![poly_1, poly_2]];
        let _: PolynomialMatrix = data.into();
    }

    #[test]
    fn test_should_create_a_matrix_successfully() {
        let ring = PolynomialRing::new(3, 5);
        let poly_1 = Polynomial::zero(&ring);
        let poly_2 = Polynomial::zero(&ring);
        let data: MatrixData = vec![vec![poly_1, poly_2]];
        let _: PolynomialMatrix = data.into();
    }

    #[test]
    fn test_get_shape() {
        let ring = PolynomialRing::new(3, 5);
        let poly_1 = Polynomial::zero(&ring);
        let data: MatrixData = vec![vec![poly_1]];
        let matrix: PolynomialMatrix = data.into();
        assert_eq!(matrix.get_shape(), (1, 1))

    }
}
