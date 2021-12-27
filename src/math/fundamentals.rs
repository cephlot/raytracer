//! Fundamental mathematical components

use std::ops::{Add, Div, Mul, Neg, Sub};

/// 3 Dimensional Tuple struct representing points or vectors.
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    /// Creates a new point Tuple
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    /// Creates a new vector Tuple
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    /// Determines if given Tuple is a vector
    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    /// Determines if given Tuple is a point
    fn is_point(&self) -> bool {
        self.w != 0.0
    }

    /// Computes the magnitude of given vector Tuple
    fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt()
    }

    /// Normalizes the given vector Tuple to a unit vector
    pub fn normalize(self) -> Tuple {
        let magnitude = self.magnitude();
        self / magnitude
    }

    /// Computes the dot product of two given vectors
    fn dot(a: &Tuple, b: &Tuple) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }

    fn cross(a: &Tuple, b: &Tuple) -> Tuple {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_type() {
        let mut t: Tuple = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert_eq!(4.3, t.x);
        assert_eq!(-4.2, t.y);
        assert_eq!(3.1, t.z);
        assert_eq!(1.0, t.w);
        assert_eq!(true, t.is_point());
        assert_eq!(false, t.is_vector());

        t = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert_eq!(4.3, t.x);
        assert_eq!(-4.2, t.y);
        assert_eq!(3.1, t.z);
        assert_eq!(0.0, t.w);
        assert_eq!(false, t.is_point());
        assert_eq!(true, t.is_vector());
    }

    #[test]
    fn should_create_correct_types() {
        let mut reference = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 1.0,
        };
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
        let a = Tuple {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };
        let b = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };
        let reference = Tuple {
            x: 1.0,
            y: 1.0,
            z: 6.0,
            w: 1.0,
        };

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
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let reference = Tuple {
            x: -1.0,
            y: 2.0,
            z: -3.0,
            w: 4.0,
        };

        assert_eq!(-a, reference);
    }

    #[test]
    fn should_multiply_by_scalar_correctly() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let reference = Tuple {
            x: 3.5,
            y: -7.0,
            z: 10.5,
            w: -14.0,
        };

        assert_eq!(a * 3.5, reference);
    }

    #[test]
    fn should_multiply_by_fraction_correctly() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let reference = Tuple {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0,
        };

        assert_eq!(a * 0.5, reference);
    }

    #[test]
    fn should_divide_by_fraction_correctly() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let reference = Tuple {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0,
        };

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
