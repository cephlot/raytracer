//! Raytracer library

pub mod math;

use math::fundamentals::Tuple;

pub struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple,
}

pub struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple,
}

impl Environment {
    pub fn tick(&self, projectile: Projectile) -> Projectile {
        let position = projectile.position + projectile.velocity;
        let velocity = projectile.velocity + self.gravity + self.wind;

        Projectile { position, velocity }
    }
}
