//! Example of a clock 

use libray::graphics::{Canvas, Color};
use libray::math::{Tuple, rotation_z};

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut canvas = Canvas::new(100, 100);
    let white = Color::new(1.0, 1.0, 1.0);

    for i in 0..=11 {
        let transform = rotation_z(i as f64 * std::f64::consts::PI/6.0).scale(400.0, 400.0, 0.0).translate(50.0, 50.0, 0.0);
        let point = transform*Tuple::point(0.0, 0.1, 0.0);
        canvas.write_pixel((point.x).ceil() as usize, ((point.y).ceil() as usize) - 1, white);
    }

    fs::write("clock.ppm", canvas.to_ppm())?;
    
    Ok(())
}