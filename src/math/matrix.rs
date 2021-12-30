//! Matrix representation and operations

use crate::math::Tuple;
use std::convert::From;
use std::ops::{Index, IndexMut, Mul};

/// Matrix representation
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    /// No. rows in the Matrix
    rows: usize,
    /// No. columns in the Matrix
    cols: usize,
    /// Internal matrix representation
    matrix: Vec<Vec<f64>>,
}

impl Matrix {
    /// Returns a new identity Matrix with given size
    ///
    /// # Arguemnts:
    ///
    /// * `rows` - no. rows in the Matrix
    /// * `cols` - no. cols in the Matrix
    pub fn new(rows: usize, cols: usize) -> Matrix {
        let mut matrix = vec![vec![0.0; cols]; rows];

        for i in 0..rows {
            matrix[i][i] = 1.0;
        }

        Matrix { rows, cols, matrix }
    }

    /// Transposes Matrix and returns a new Matrix
    pub fn transpose(&self) -> Matrix {
        let mut new = vec![vec![0.0; self.rows]; self.cols];

        for i in 0..self.rows {
            for j in 0..self.cols {
                new[i][j] = self[(j, i)]
            }
        }

        Matrix::from(new)
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        if self.matrix.len() != 4 && self.matrix[0].len() != 4 {
            panic!("Incorrect matrix shape");
        }

        let mut m = Matrix::new(self.matrix.len(), self.matrix[0].len());

        for i in 0..4 {
            for j in 0..4 {
                m[(i, j)] = self[(i, 0)] * rhs[(0, j)]
                    + self[(i, 1)] * rhs[(1, j)]
                    + self[(i, 2)] * rhs[(2, j)]
                    + self[(i, 3)] * rhs[(3, j)];
            }
        }

        m
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        if self.matrix.len() != 4 || self.matrix[0].len() != 4 {
            panic!("Incorrect matrix shape");
        }

        let mut v = vec![];

        for i in 0..4 {
            let mut sum = 0.0;
            for j in 0..4 {
                sum += self[(i, j)] * rhs[j];
            }
            v.push(sum);
        }

        Tuple::from(v)
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, idx: (usize, usize)) -> &f64 {
        &self.matrix[idx.0][idx.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut f64 {
        &mut self.matrix[idx.0][idx.1]
    }
}

impl From<Vec<Vec<f64>>> for Matrix {
    fn from(matrix: Vec<Vec<f64>>) -> Matrix {
        Matrix {
            rows: matrix.len(),
            cols: matrix[0].len(),
            matrix,
        }
    }
}

impl From<Vec<f64>> for Matrix {
    fn from(vec: Vec<f64>) -> Matrix {
        Matrix {
            rows: 1,
            cols: vec.len(),
            matrix: vec![vec],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Tuple;

    #[test]
    fn should_contain_correct_data() {
        let a = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ];
        let m = Matrix::from(a);

        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 3)], 4.0);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.0);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
    }

    #[test]
    fn should_be_able_to_create_multiple_dimensions() {
        let a = vec![vec![-3.0, 5.0], vec![1.0, -2.0]];
        let m = Matrix::from(a);

        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(0, 1)], 5.0);
        assert_eq!(m[(1, 0)], 1.0);
        assert_eq!(m[(1, 1)], -2.0);

        let a = vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ];
        let m = Matrix::from(a);

        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(1, 1)], -2.0);
        assert_eq!(m[(2, 2)], 1.0);
    }

    #[test]
    fn should_be_equal_and_not_equal() {
        let m = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ];
        let a = Matrix::from(m.clone());
        let b = Matrix::from(m.clone());

        assert_eq!(a, b);

        let n = vec![
            vec![2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0],
            vec![8.0, 7.0, 6.0, 5.0],
            vec![4.0, 3.0, 2.0, 1.0],
        ];

        let b = Matrix::from(n.clone());

        assert_ne!(a, b);
    }

    #[test]
    fn multiply_should_compute_correctly() {
        let a = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ];
        let b = vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ];
        let c = vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ];

        let a = Matrix::from(a);
        let b = Matrix::from(b);
        let c = Matrix::from(c);

        assert_eq!(a * b, c);
    }

    #[test]
    fn should_be_able_tomultiply_by_tuple() {
        let a = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];

        let a = Matrix::from(a);
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let reference = Tuple::new(18.0, 24.0, 33.0, 1.0);

        assert_eq!(reference, a * b);
    }

    #[test]
    fn multiplying_with_identity_is_the_same() {
        let a = vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ];
        let reference = Matrix::from(a.clone());
        let a = Matrix::from(a);
        let identity = Matrix::new(4, 4);

        assert_eq!(reference, a * identity);
    }

    #[test]
    fn should_transpose_correctly() {
        let a = vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ];
        let a = Matrix::from(a);

        let reference = vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ];
        let reference = Matrix::from(reference);

        assert_eq!(reference, a.transpose());
    }
}
