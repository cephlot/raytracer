use super::Tuple;

/// Representing individual ray
pub struct Ray {
    /// Point of origin of the ray
    origin: Tuple,
    /// Direction vector of the ray
    direction: Tuple,
}

impl Ray {
    /// Returns a new ray with an origin and a direction
    ///
    /// # Arguments
    ///
    /// * `origin` - point of origin of the ray
    /// * `direction` - direction vector of the ray
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    /// Returns the position as a point tuple of ray at time t
    ///
    /// # Arguments
    ///
    /// * `t` - time to check position at
    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_ray_correctly() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::point(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn position_should_return_correct_position_at_certain_times() {
        let ray = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));

        assert_eq!(Tuple::point(2.0, 3.0, 4.0), ray.position(0.0));
        assert_eq!(Tuple::point(3.0, 3.0, 4.0), ray.position(1.0));
        assert_eq!(Tuple::point(1.0, 3.0, 4.0), ray.position(-1.0));
        assert_eq!(Tuple::point(4.5, 3.0, 4.0), ray.position(2.5));
    }
}
