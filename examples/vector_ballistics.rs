//! Example of vector ballistics

use libray::{graphics, math::Tuple};
use std::fs;

/// Projectile struct
#[derive(Debug, Clone, Copy)]
pub struct Projectile {
    /// Tuple representing projectile position
    pub position: Tuple,
    /// Tuple representing projectile velocity
    pub velocity: Tuple,
}

/// Environment struct
#[derive(Debug, Clone, Copy)]
pub struct Environment {
    /// Tuple representing environment gravity
    pub gravity: Tuple,
    /// Tuple representing environment gravity wind
    pub wind: Tuple,
}

impl Environment {
    /// Tick environment one step
    ///
    /// # Arguments:
    ///
    /// * `projectile` - Projectile to be affected by environment properties
    pub fn tick(&self, projectile: Projectile) -> Projectile {
        let position = projectile.position + projectile.velocity;
        let velocity = projectile.velocity + self.gravity + self.wind;

        Projectile { position, velocity }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };
    let mut c = graphics::Canvas::new(900, 550);
    let red = graphics::Color::new(1.0, 0.0, 0.0);

    while p.position.y > 0.0 {
        p = e.tick(p);
        c.write_pixel(
            ((p.position.x).ceil() as usize) - 1,
            (550 - ((p.position.y).ceil() as usize)) - 1,
            red,
        );
    }

    fs::write("ballistics.ppm", c.to_ppm())?;

    Ok(())
}
