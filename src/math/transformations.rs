//! Matrix tranformation operation

use super::matrix::Matrix;
use super::tuple::Tuple;

/// Returns a translation matrix with the given translation units
/// 
/// # Arguments:
/// 
/// * `x` - units in x axis
/// * `y` - units in y axis
/// * `z` - units in z axis
pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = Matrix::new(4, 4);

    m[(0, 3)] = x;
    m[(1, 3)] = y;
    m[(2, 3)] = z;

    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_a_translation_matrix_correctly() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(transform*p, Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn should_get_correct_point_when_multiplying_its_inverse() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(transform.inverse()*p, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn transform_should_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(transform*p, Tuple::vector(-3.0, 4.0, 5.0));
    }
}