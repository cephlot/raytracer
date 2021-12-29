//! Matrix representation and operations

use std::ops::Index;

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
    /// Returns a new Matrix with given size
    /// 
    /// # Arguemnts:
    /// 
    /// * `rows` - no. rows in the Matrix
    /// * `cols` - no. cols in the Matrix
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix { rows, cols, matrix: vec![vec![0.0]] }
    }

    /// Returns a new Matrix from given matrix
    /// 
    /// # Arguments:
    /// 
    /// * `matrix` - matrix to construct Matrix from
    pub fn with_matrix(matrix: Vec<Vec<f64>>) -> Matrix {
        Matrix { rows: matrix.len(), cols: matrix[0].len(), matrix }
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, idx: (usize, usize)) -> &f64 {
        &self.matrix[idx.0][idx.1]
    }
}

// |  1   |  2   |  3   |  4   |
// ​ 	    |  5.5 |  6.5 |  7.5 |  8.5 |
// ​ 	    |  9   | 10   | 11   | 12   |
// ​ 	    | 13.5 | 14.5 | 15.5 | 16.5 |
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        let a = vec![
            vec![1.0, 2.0, 3.0, 4.0], 
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5]
        ];
        let m = Matrix::with_matrix(a);

        assert_eq!(m[(0,0)], 1.0);
    }
}