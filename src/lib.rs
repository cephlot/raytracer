//! Ray tracing library

/// Math module
pub mod math {
    #![deny(missing_docs,
        missing_debug_implementations, 
        missing_copy_implementations,
        trivial_casts, 
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, 
        unused_qualifications)]
    
    mod tuple;

    pub use tuple::Tuple;
}

/// Graphics module
pub mod graphics {
    #![deny(missing_docs,
        missing_debug_implementations, 
        missing_copy_implementations,
        trivial_casts, 
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, 
        unused_qualifications)]

    mod color;
    mod canvas;

    pub use color::Color;
    pub use canvas::Canvas;
}

use math::Tuple;

/// Projectile struct
#[derive(Debug)]
pub struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple,
}

/// Environment struct
#[derive(Debug)]
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
