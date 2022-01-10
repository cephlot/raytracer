use super::{Sphere, Tuple};

/// Returns the normal vector of given sphere at given point
///
/// # Arguments
///
/// `s` - sphere to find normal of
/// `p` - point tuple to calculate normal vector at
pub fn normal_at(s: Sphere, p: Tuple) -> Tuple {
    let obj_point = s.transform.inverse() * p;
    let obj_norm = obj_point - Tuple::point(0.0, 0.0, 0.0);
    let mut world_normal = s.transform.inverse().transpose() * obj_norm;
    world_normal.w = 0.0;

    world_normal
}

#[cfg(test)]
mod tests {
    use super::super::{transformations, Matrix};
    use super::*;

    #[test]
    fn should_calculate_the_correct_normal_vector() {
        let s = Sphere::new();
        let n = normal_at(s, Tuple::point(1.0, 0.0, 0.0));
        let reference = Tuple::vector(1.0, 0.0, 0.0);

        assert_eq!(reference, n);

        let s = Sphere::new();
        let n = normal_at(s, Tuple::point(0.0, 1.0, 0.0));
        let reference = Tuple::vector(0.0, 1.0, 0.0);

        assert_eq!(reference, n);

        let s = Sphere::new();
        let n = normal_at(s, Tuple::point(0.0, 0.0, 1.0));
        let reference = Tuple::vector(0.0, 0.0, 1.0);

        assert_eq!(reference, n);

        let s = Sphere::new();
        let n = normal_at(
            s,
            Tuple::point(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            ),
        );
        let reference = Tuple::vector(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );

        assert_eq!(reference, n);
    }

    #[test]
    fn normal_vector_should_be_normalized() {
        let s = Sphere::new();
        let n = normal_at(
            s,
            Tuple::point(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            ),
        );
        let reference = n.clone().normalize();

        assert_eq!(reference, n);
    }

    #[test]
    fn should_compute_correct_normal_on_translated_sphere() {
        let mut s = Sphere::new();
        s.transform(transformations::translation(0.0, 1.0, 0.0));
        let n = normal_at(s, Tuple::point(0.0, 1.70711, -0.70711));
        let reference = Tuple::vector(0.0, 0.70711, -0.70711);

        assert_eq!(reference, n);
    }

    #[test]
    fn should_compute_correct_normal_on_transformed_sphere() {
        let mut s = Sphere::new();
        s.transform(Matrix::new(4,4).scale(1.0, 0.5, 1.0)*transformations::rotation_z(std::f64::consts::PI / 5.0));
        let n = normal_at(
            s,
            Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -(2.0_f64.sqrt() / 2.0)),
        );
        let reference = Tuple::vector(0.0, 0.97014, -0.24254);

        assert_eq!(reference, n);
    }
}
