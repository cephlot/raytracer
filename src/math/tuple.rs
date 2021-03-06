//! Matrix representation and operations
//!
use std::convert::From;
use std::ops::{Add, Div, Index, Mul, Neg, Sub};

const EPSILON: f64 = 0.00001;

/// 3 Dimensional Tuple struct representing points or vectors.
#[derive(PartialOrd, Debug, Clone, Copy)]
pub struct Tuple {
    /// x-value of the tuple
    pub x: f64,
    /// y-value of the tuple
    pub y: f64,
    /// z-value of the tuple
    pub z: f64,
    /// w-value of the tuple. A non-zero value indicates a point, otherwise a
    /// vector
    pub w: f64,
    _private: (),
}

impl Tuple {
    /// Creates a new Tuple
    ///
    /// # Arguments:
    ///
    /// * `x` - x value of tuple
    /// * `y` - y value of tuple
    /// * `z` - z value of tuple
    /// * `w` - w value of tuple
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w,
            _private: (),
        }
    }

    /// Creates a new point Tuple where `w` is `1.0`
    ///
    /// # Arguments:
    ///
    /// * `x` - x value of point
    /// * `y` - y value of point
    /// * `z` - z value of point
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: 1.0,
            _private: (),
        }
    }

    /// Creates a new vector Tuple where `w` is `0.0`
    ///
    /// # Arguments:
    ///
    /// * `x` - x value of point
    /// * `y` - y value of point
    /// * `z` - z value of point
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: 0.0,
            _private: (),
        }
    }

    /// Determines if given Tuple is a vector
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    /// Determines if given Tuple is a point
    pub fn is_point(&self) -> bool {
        self.w != 0.0
    }

    /// Computes the magnitude of given vector Tuple using Pythagoras' theorem
    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt()
    }

    /// Normalizes the given vector Tuple to a unit vector
    pub fn normalize(self) -> Tuple {
        self / self.magnitude()
    }

    /// Computes the dot product of two given vectors and returns a float
    ///
    /// # Arguments
    ///
    /// * `a` - vector of LHS
    /// * `b` - vector of RHS
    pub fn dot(a: &Tuple, b: &Tuple) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }

    /// Computes the cross product of two given vectors and returns a vector
    ///
    /// # Arguments
    ///
    /// * `a` - vector of LHS
    /// * `b` - vector of RHS
    pub fn cross(a: &Tuple, b: &Tuple) -> Tuple {
        Tuple::vector(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(mut self, other: Tuple) -> Tuple {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;

        self
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(mut self, other: Tuple) -> Tuple {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;

        self
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(mut self) -> Tuple {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self.w = -self.w;

        self
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(mut self, rhs: f64) -> Tuple {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;

        self
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(mut self, rhs: f64) -> Tuple {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;

        self
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
            && (self.w - other.w).abs() < EPSILON
    }
}

impl Index<usize> for Tuple {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Incorrect index"),
        }
    }
}

impl From<Vec<f64>> for Tuple {
    fn from(vec: Vec<f64>) -> Tuple {
        if vec.len() != 4 {
            panic!("Incorrect vector length")
        }

        Tuple::new(vec[0], vec[1], vec[2], vec[3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_type() {
        let mut t: Tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(4.3, t.x);
        assert_eq!(-4.2, t.y);
        assert_eq!(3.1, t.z);
        assert_eq!(1.0, t.w);
        assert_eq!(true, t.is_point());
        assert_eq!(false, t.is_vector());

        t = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(4.3, t.x);
        assert_eq!(-4.2, t.y);
        assert_eq!(3.1, t.z);
        assert_eq!(0.0, t.w);
        assert_eq!(false, t.is_point());
        assert_eq!(true, t.is_vector());
    }

    #[test]
    fn should_create_correct_types() {
        let mut reference = Tuple::new(4.0, -4.0, 3.0, 1.0);
        let mut t = Tuple::point(4.0, -4.0, 3.0);
        assert_eq!(true, t.is_point());
        assert_eq!(reference, t);

        reference.w = 0.0;
        t = Tuple::vector(4.0, -4.0, 3.0);
        assert_eq!(true, t.is_vector());
        assert_eq!(reference, t);
    }

    #[test]
    fn should_add_components_correctly() {
        let a = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let b = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let reference = Tuple::new(1.0, 1.0, 6.0, 1.0);

        assert_eq!(a + b, reference);
    }

    #[test]
    fn should_subtract_components_correctly_and_yield_vector() {
        let a = Tuple::point(3.0, 2.0, 1.0);
        let b = Tuple::point(5.0, 6.0, 7.0);
        let reference = Tuple::vector(-2.0, -4.0, -6.0);

        assert_eq!(a - b, reference);
    }

    #[test]
    fn should_subtract_components_correctly_and_yield_point() {
        let a = Tuple::point(3.0, 2.0, 1.0);
        let b = Tuple::vector(5.0, 6.0, 7.0);
        let reference = Tuple::point(-2.0, -4.0, -6.0);

        assert_eq!(a - b, reference);
    }

    #[test]
    fn should_subtract_vector_components_correctly_and_yield_vector() {
        let a = Tuple::vector(3.0, 2.0, 1.0);
        let b = Tuple::vector(5.0, 6.0, 7.0);
        let reference = Tuple::vector(-2.0, -4.0, -6.0);

        assert_eq!(a - b, reference);
    }

    #[test]
    fn should_subtract_vector_from_zero_correctly() {
        let a = Tuple::vector(1.0, -2.0, 3.0);
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let reference = Tuple::vector(-1.0, 2.0, -3.0);

        assert_eq!(zero - a, reference);
    }

    #[test]
    fn should_negate_tuple_properly() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let reference = Tuple::new(-1.0, 2.0, -3.0, 4.0);

        assert_eq!(-a, reference);
    }

    #[test]
    fn should_multiply_by_scalar_correctly() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let reference = Tuple::new(3.5, -7.0, 10.5, -14.0);

        assert_eq!(a * 3.5, reference);
    }

    #[test]
    fn should_multiply_by_fraction_correctly() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let reference = Tuple::new(0.5, -1.0, 1.5, -2.0);

        assert_eq!(a * 0.5, reference);
    }

    #[test]
    fn should_divide_by_fraction_correctly() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let reference = Tuple::new(0.5, -1.0, 1.5, -2.0);

        assert_eq!(a / 2.0, reference);
    }

    #[test]
    fn should_compute_magnitude_correctly() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(1.0, v.magnitude());

        let v = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(1.0, v.magnitude());

        let v = Tuple::vector(0.0, 0.0, 1.0);
        assert_eq!(1.0, v.magnitude());

        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(14.0_f64.sqrt(), v.magnitude());

        let v = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(14.0_f64.sqrt(), v.magnitude());
    }

    #[test]
    fn should_normalize_vector_correctly() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        let reference = Tuple::vector(1.0, 0.0, 0.0);

        assert_eq!(v.normalize(), reference);

        let v = Tuple::vector(1.0, 2.0, 3.0);
        let reference = Tuple::vector(
            1.0 / 14.0_f64.sqrt(),
            2.0 / 14.0_f64.sqrt(),
            3.0 / 14.0_f64.sqrt(),
        );

        assert_eq!(v.normalize(), reference);

        let v = Tuple::vector(1.0, 2.0, 3.0);

        assert_eq!(v.normalize().magnitude(), 1.0);
    }

    #[test]
    fn should_compute_dot_product_correctly() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(Tuple::dot(&a, &b), 20.0);
    }

    #[test]
    fn should_compute_cross_product_correctly() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        let reference = Tuple::vector(-1.0, 2.0, -1.0);

        assert_eq!(Tuple::cross(&a, &b), reference);

        let reference = Tuple::vector(1.0, -2.0, 1.0);

        assert_eq!(Tuple::cross(&b, &a), reference);
    }
}
