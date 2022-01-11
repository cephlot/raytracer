//! Example of a purple shaded sphere on a black canvas with a single point 
//! light

use libray::graphics::{Canvas, Color, Light};
use libray::math::{Intersection, Ray, Sphere, Tuple, normal_at};

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut canvas = Canvas::new(100, 100);
    let mut s = Sphere::new();
    s.material.color = Color::new(1.0, 0.2, 0.7); 
    let pixel_size = 7.0 / 100.0;
    let half = 7.0 / 2.0;
    let light = Light::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    for y in 0..100 {
        let world_y = half - pixel_size * (y as f64);

        for x in 0..100 {
            let world_x = -half + pixel_size * (x as f64);
            let position = Tuple::point(world_x, world_y, 10.0);

            let ray = Ray::new(
                Tuple::point(0.0, 0.0, -5.0),
                (position - Tuple::point(0.0, 0.0, -5.0)).normalize(),
            );
            let intersections = ray.intersect(&s);

            match Intersection::hit(&intersections) {
                Some(hit) => {
                    let eye = -ray.direction;
                    let point = ray.position(hit.t);
                    let normal = normal_at(hit.sphere.clone(), point);
                    let color = s.material.lighting(light, point, eye, normal);

                    canvas.write_pixel(x, y, color)
                },
                None => (),
            }
        }
    }

    fs::write("shaded_sphere.ppm", canvas.to_ppm())?;

    Ok(())
}
