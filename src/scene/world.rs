
use crate::math::{Sphere, Tuple, Ray, scaling};
use crate::graphics::{Light, Color, Material};

/// Representing world container
#[derive(Debug, Clone)]
pub struct World {
    /// Vector of world objects
    pub objects: Vec<Sphere>,
    /// Vector of light sources
    lights: Vec<Light>,
}

impl World {
    /// Returns a new empty world
    pub fn new() -> World {
        World { objects: vec![], lights: vec![] }
    }

    
}
impl Default for World {
    fn default() -> World {
        let mut a = Sphere::new();
        let mut material = Material::new();
        material.color = Color::new(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        a.material = material;
        let mut b = Sphere::new();
        b.transform(scaling(0.5, 0.5, 0.5));

        let objects = vec![a, b];
        let lights = vec![Light::new(Tuple::point(-10.0, 10.0, -10.0), Color::WHITE)];

        World { objects, lights }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::*;

    #[test]
    fn new_world_should_not_contain_anything() {
        let w = World::new();

        assert_eq!(0, w.lights.len());
        assert_eq!(0, w.objects.len());
    }

    #[test]
    fn default_world_should_contain_correct_lights_and_objects() {
        let w = World::default();
        let light = Light::new(Tuple::point(-10.0, 10.0, -10.0), Color::WHITE);
        let mut a = Sphere::new();
        let mut material = Material::new();
        material.color = Color::new(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        a.material = material;
        let mut b = Sphere::new();
        b.transform(scaling(0.5, 0.5, 0.5));

        assert_eq!(true, w.lights.contains(&light));
        assert_eq!(true, w.objects.contains(&a));
        assert_eq!(true, w.objects.contains(&b));
    }

    #[test]
    fn ray_should_intersect_world_object_correctly() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let intersections = r.intersect_world(w);

        assert_eq!(4, intersections.len());
        assert_eq!(4.0, intersections[0].t);
        assert_eq!(4.5, intersections[1].t);
        assert_eq!(5.5, intersections[2].t);
        assert_eq!(6.0, intersections[3].t);
    }
}