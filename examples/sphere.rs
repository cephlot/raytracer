//! Example of a red sphere on a black canvas

use libray::graphics::{Canvas, Color};
use libray::math::{Intersection, Ray, Sphere, Tuple};

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut canvas = Canvas::new(100, 100);
    let red = Color::new(1.0, 0.0, 0.0);
    let s = Sphere::new();
    let pixel_size = 7.0 / 100.0;
    let half = 7.0 / 2.0;

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
                Some(_) => canvas.write_pixel(x, y, red),
                None => (),
            }
        }
    }

    fs::write("sphere.ppm", canvas.to_ppm())?;

    Ok(())
}
