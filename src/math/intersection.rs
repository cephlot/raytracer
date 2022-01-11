use super::Matrix;
use super::Tuple;
use crate::graphics::Material;

/// Represents an individual ray
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray {
    /// Point of origin of the ray
    origin: Tuple,
    /// Direction vector of the ray
    pub direction: Tuple,
}

/// Represents a sphere object
#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    /// Origin point of the sphere
    origin: Tuple,
    /// Radius of the sphere
    radius: f64,
    /// Transformation matrix of the Sphere
    pub transform: Matrix,
    /// Material of the sphere
    pub material: Material,
}

/// Aggregation of time and object that was intersected
#[derive(Debug, PartialEq, Clone)]
pub struct Intersection<'a> {
    /// Time where an object was
    pub t: f64,
    /// Reference to intersected object
    pub sphere: &'a Sphere,
    _private: ()
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

    /// Calculates and returns the points at which the ray intersects a given
    /// sphere
    ///
    /// # Arguments
    ///
    /// * `s` - sphere to calculate intersections for
    pub fn intersect<'a>(&self, s: &'a Sphere) -> Vec<Intersection<'a>> {
        let r = self.transform(s.transform.inverse());
        let v = r.origin - s.origin;
        let a = Tuple::dot(&r.direction, &r.direction);
        let b = 2.0 * Tuple::dot(&r.direction, &v);
        let c = Tuple::dot(&v, &v) - 1.0;
        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        let first = (-b - discriminant.sqrt()) / (2.0 * a);
        let second = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![Intersection::new(first, &s), Intersection::new(second, &s)]
    }

    /// Returns a new ray transformed by the given transformation matrix
    ///
    /// # Arguments
    ///
    /// * `matrix` - transformation matrix to affect ray by
    pub fn transform(&self, matrix: Matrix) -> Ray {
        Ray {
            origin: matrix.clone() * self.origin.clone(),
            direction: matrix * self.direction.clone(),
        }
    }
}

impl Sphere {
    /// Returns a new sphere object
    ///
    /// # Arguments
    ///
    /// * `origin` - origin point of the Sphere
    pub fn new() -> Sphere {
        Sphere {
            origin: Tuple::point(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Matrix::new(4, 4),
            material: Material::new(),
        }
    }

    /// Sets the transformation matrix of sphere to given transformation matrix
    ///
    /// # Arguments
    ///
    /// * `transform` - transformation matrix to set for sphere
    pub fn transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
}

impl<'a> Intersection<'a> {
    /// Returns a new intersection
    ///
    /// # Arguments
    ///
    /// * `sphere` - reference to intersected object
    pub fn new(t: f64, sphere: &'a Sphere) -> Intersection {
        Intersection { t, sphere, _private: ()}
    }

    /// Returns the first nonnegative intersection as a hit
    ///
    /// # Arguments
    ///
    /// * `intersections` - vector of intersections to sort from
    pub fn hit<'b>(intersections: &'b Vec<Intersection>) -> Option<Intersection<'b>> {
        let tmp: Vec<Intersection> = intersections
            .clone()
            .into_iter()
            .filter(|i| i.t >= 0.0)
            .collect();

        if tmp.len() == 0 {
            return None;
        }

        let mut min = tmp[0].clone();

        for i in tmp {
            if i.t < min.t {
                min = i;
            }
        }

        Some(min)
    }
}

impl PartialOrd for Intersection<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

#[cfg(test)]
mod tests {
    use super::super::transformations;
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
    fn should_create_sphere_correctly() {
        let s = Sphere::new();

        assert_eq!(Tuple::point(0.0, 0.0, 0.0), s.origin);
        assert_eq!(1.0, s.radius);
        assert_eq!(Matrix::new(4, 4), s.transform);
        assert_eq!(Material::new(), s.material);
    }

    #[test]
    fn position_should_return_correct_position_at_certain_times() {
        let ray = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));

        assert_eq!(Tuple::point(2.0, 3.0, 4.0), ray.position(0.0));
        assert_eq!(Tuple::point(3.0, 3.0, 4.0), ray.position(1.0));
        assert_eq!(Tuple::point(1.0, 3.0, 4.0), ray.position(-1.0));
        assert_eq!(Tuple::point(4.5, 3.0, 4.0), ray.position(2.5));
    }

    #[test]
    fn should_return_correct_intersection_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let intersections = r.intersect(&s);

        assert_eq!(2, intersections.len());
        assert_eq!(4.0, intersections[0].t);
        assert_eq!(6.0, intersections[1].t);
    }

    #[test]
    fn tangent_ray_should_have_same_intersection_points() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let intersections = r.intersect(&s);

        assert_eq!(2, intersections.len());
        assert_eq!(5.0, intersections[0].t);
        assert_eq!(5.0, intersections[1].t);
    }

    #[test]
    fn should_have_zero_intersections_when_ray_misses_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let intersections = r.intersect(&s);

        assert_eq!(0, intersections.len());
    }

    #[test]
    fn should_have_two_intersections_when_ray_origininates_inside_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let intersections = r.intersect(&s);

        assert_eq!(2, intersections.len());
        assert_eq!(-1.0, intersections[0].t);
        assert_eq!(1.0, intersections[1].t);
    }

    #[test]
    fn should_have_two_intersections_when_ray_origininates_front_of_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let intersections = r.intersect(&s);

        assert_eq!(2, intersections.len());
        assert_eq!(-6.0, intersections[0].t);
        assert_eq!(-4.0, intersections[1].t);
    }

    #[test]
    fn intersection_should_contain_correct_values() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(3.5, i.t);
        assert_eq!(&s, i.sphere);
    }

    #[test]
    fn should_aggregate_intersections_correctly() {
        let s = Sphere::new();
        let a = Intersection::new(1.0, &s);
        let b = Intersection::new(2.0, &s);
        let intersections = vec![a, b];

        assert_eq!(2, intersections.len());
        assert_eq!(1.0, intersections[0].t);
        assert_eq!(2.0, intersections[1].t);
    }

    #[test]
    fn should_contain_sphere_object_reference() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let intersections = r.intersect(&s);

        assert_eq!(2, intersections.len());
        assert_eq!(&s, intersections[0].sphere);
        assert_eq!(&s, intersections[1].sphere);
    }

    #[test]
    fn should_compute_the_correct_hit() {
        let s = Sphere::new();
        let a = Intersection::new(1.0, &s);
        let b = Intersection::new(2.0, &s);
        let intersections = vec![a.clone(), b.clone()];
        let hit = Intersection::hit(&intersections);

        assert_eq!(Some(a), hit);

        let a = Intersection::new(-1.0, &s);
        let b = Intersection::new(1.0, &s);
        let intersections = vec![a.clone(), b.clone()];
        let hit = Intersection::hit(&intersections);

        assert_eq!(Some(b), hit);

        let a = Intersection::new(-2.0, &s);
        let b = Intersection::new(-1.0, &s);
        let intersections = vec![a.clone(), b.clone()];
        let hit = Intersection::hit(&intersections);

        assert_eq!(None, hit);

        let a = Intersection::new(5.0, &s);
        let b = Intersection::new(7.0, &s);
        let c = Intersection::new(-3.0, &s);
        let d = Intersection::new(2.0, &s);
        let intersections = vec![a.clone(), b.clone(), c.clone(), d.clone()];
        let hit = Intersection::hit(&intersections);

        assert_eq!(Some(d), hit);
    }

    #[test]
    fn should_be_able_to_translate_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = transformations::translation(3.0, 4.0, 5.0);
        let r = r.transform(m);

        assert_eq!(Tuple::point(4.0, 6.0, 8.0), r.origin);
        assert_eq!(Tuple::vector(0.0, 1.0, 0.0), r.direction);
    }

    #[test]
    fn should_be_able_to_scale_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = transformations::scaling(2.0, 3.0, 4.0);
        let r = r.transform(m);

        assert_eq!(Tuple::point(2.0, 6.0, 12.0), r.origin);
        assert_eq!(Tuple::vector(0.0, 3.0, 0.0), r.direction);
    }

    #[test]
    fn sphere_should_contain_correct_transformation() {
        let s = Sphere::new();
        assert_eq!(s.transform, Matrix::new(4, 4));

        let mut s = Sphere::new();
        let t = transformations::translation(2.0, 3.0, 4.0);
        s.transform(t.clone());
        assert_eq!(s.transform, t);
    }

    #[test]
    fn should_intersect_a_scaled_sphere_correctly() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform(transformations::scaling(2.0, 2.0, 2.0));
        let intersections = r.intersect(&s);

        assert_eq!(2, intersections.len());
        assert_eq!(3.0, intersections[0].t);
        assert_eq!(7.0, intersections[1].t);
    }

    #[test]
    fn should_intersect_a_translated_sphere_correctly() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform(transformations::translation(5.0, 0.0, 0.0));
        let intersections = r.intersect(&s);

        assert_eq!(0, intersections.len());
    }
}
