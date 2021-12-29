//! Binary example

use libray::{graphics, math::Tuple, Environment, Projectile};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };
    let mut i = 0;
    let mut c = graphics::Canvas::new(900, 550);
    let red = graphics::Color::new(1.0, 0.0, 0.0);

    while p.position.y > 0.0 {
        p = e.tick(p);
        i += 1;
        println!("{}: {:?}", i, p.position);
        c.write_pixel(
            ((p.position.x).ceil() as usize) - 1,
            (550 - ((p.position.y).ceil() as usize)) - 1,
            red,
        );
    }

    fs::write("ballistics.ppm", c.to_ppm())?;

    Ok(())
}
