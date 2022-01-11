//! Math module
//!
//! Contains fundamental math type and methods.

mod intersection;
mod matrix;
mod transformations;
mod tuple;
mod vectors;

pub use intersection::{Intersection, Ray, Sphere};
pub use matrix::Matrix;
pub use transformations::*;
pub use tuple::Tuple;
pub use vectors::{normal_at, reflect};
