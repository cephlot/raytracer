//! Matrix tranformation operation

use super::matrix::Matrix;

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

/// Returns a scling matrix with the given scale units
///
/// # Arguments:
///
/// * `x` - scaling in x axis
/// * `y` - scaling in y axis
/// * `z` - scaling in z axis
pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = Matrix::new(4, 4);

    m[(0, 0)] = x;
    m[(1, 1)] = y;
    m[(2, 2)] = z;

    m
}

/// Returns a rotation matrix around the x axis
///
/// # Arguments:
///
/// * `rads` - radians to rotate
pub fn rotation_x(rads: f64) -> Matrix {
    let mut m = Matrix::new(4, 4);

    m[(1, 1)] = rads.cos();
    m[(2, 1)] = rads.sin();
    m[(1, 2)] = -rads.sin();
    m[(2, 2)] = rads.cos();

    m
}

/// Returns a rotation matrix around the y axis
///
/// # Arguments:
///
/// * `rads` - radians to rotate
pub fn rotation_y(rads: f64) -> Matrix {
    let mut m = Matrix::new(4, 4);

    m[(0, 0)] = rads.cos();
    m[(2, 0)] = -rads.sin();
    m[(0, 2)] = rads.sin();
    m[(2, 2)] = rads.cos();

    m
}

/// Returns a rotation matrix around the z axis
///
/// # Arguments:
///
/// * `rads` - radians to rotate
pub fn rotation_z(rads: f64) -> Matrix {
    let mut m = Matrix::new(4, 4);

    m[(0, 0)] = rads.cos();
    m[(1, 0)] = rads.sin();
    m[(0, 1)] = -rads.sin();
    m[(1, 1)] = rads.cos();

    m
}

///
pub fn skewing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut m = Matrix::new(4, 4);

    m[(1,0)] = yx;
    m[(2,0)] = zx;
    m[(0,1)] = xy;
    m[(2,1)] = zy;
    m[(0,2)] = xz;
    m[(1,2)] = yz;
    
    m
}

#[cfg(test)]
mod tests {
    use super::super::tuple::Tuple;
    use super::*;

    #[test]
    fn should_create_a_translation_matrix_correctly() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(transform * p, Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn should_get_correct_point_when_multiplying_its_inverse() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(transform.inverse() * p, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn transform_should_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(transform * p, Tuple::vector(-3.0, 4.0, 5.0));
    }

    #[test]
    fn scaling_should_apply_correctly() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        assert_eq!(transform * p, Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_should_apply_to_vectors_correctly() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(transform * v, Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn should_multiply_with_inverse_correctly() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(transform.inverse() * v, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn scaling_should_reflect_correctly() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let v = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(transform.inverse() * v, Tuple::vector(-2.0, 3.0, 4.0));
    }

    #[test]
    fn should_rotate_correctly_around_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let a = rotation_x(std::f64::consts::PI / 4.0);
        let b = rotation_x(std::f64::consts::PI / 2.0);

        assert_eq!(
            a * p,
            Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(b * p, Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_rotation_should_rotate_in_opposite_direction() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let a = rotation_x(std::f64::consts::PI / 4.0);

        assert_eq!(
            a.inverse() * p,
            Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn should_rotate_correctly_around_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let a = rotation_y(std::f64::consts::PI / 4.0);
        let b = rotation_y(std::f64::consts::PI / 2.0);

        assert_eq!(
            a * p,
            Tuple::point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(b * p, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn should_rotate_correctly_around_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let a = rotation_z(std::f64::consts::PI / 4.0);
        let b = rotation_z(std::f64::consts::PI / 2.0);

        assert_eq!(
            a * p,
            Tuple::point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(b * p, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn skewing_should_transform_correctly() {
        let transform = skewing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        
        assert_eq!(transform*p, Tuple::point(5.0, 3.0, 4.0));

        let transform = skewing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        
        assert_eq!(transform*p, Tuple::point(6.0, 3.0, 4.0));

        let transform = skewing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        
        assert_eq!(transform*p, Tuple::point(2.0, 5.0, 4.0));

        let transform = skewing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        
        assert_eq!(transform*p, Tuple::point(2.0, 7.0, 4.0));

        let transform = skewing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        
        assert_eq!(transform*p, Tuple::point(2.0, 3.0, 6.0));

        let transform = skewing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        
        assert_eq!(transform*p, Tuple::point(2.0, 3.0, 7.0));
    }
}
