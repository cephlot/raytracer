//! Ray tracing library
//!
//! This is an implementation of The Ray Tracer Challenge by Jamis Buck
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

pub mod graphics;
pub mod math;

use math::Tuple;

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
