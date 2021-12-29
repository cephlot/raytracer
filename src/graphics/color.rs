//! Fundamental color component 

use std::ops::{Add, Div, Mul, Neg, Sub};

/// Three-dimensional color representation
#[derive(Debug, Clone, Copy)]
pub struct Color {
    /// Red value
    pub r: f64,
    /// Green value
    pub g: f64,
    /// Blue value
    pub b: f64,
}

impl Color {
    /// Creates a new color
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }
}

impl Default for Color {
    fn default() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0 }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(mut self, other: Color) -> Color {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;

        self
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(mut self, other: Color) -> Color {
        self.r -= other.r;
        self.g -= other.g;
        self.b -= other.b;

        self
    }
}

impl Neg for Color {
    type Output = Color;

    fn neg(mut self) -> Color {
        self.r = -self.r;
        self.g = -self.g;
        self.b = -self.b;

        self
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(mut self, rhs: f64) -> Color {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;

        self
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(mut self, rhs: Color) -> Color {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;

        self
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(mut self, rhs: f64) -> Color {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;

        self
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        (self.r - other.r).abs() < f64::EPSILON &&
        (self.g - other.g).abs() < f64::EPSILON &&
        (self.b - other.b).abs() < f64::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_color_contains_correct_values() {
        let c = Color::new(-0.5, 0.4, 1.7);
        
        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }

    #[test]
    fn adding_colors_should_compute_correctly() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);
        let reference = Color::new(1.6, 0.7, 1.0);
        
        assert_eq!(reference, a+b);
    }

    #[test]
    fn subtracting_colors_should_compute_correctly() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);
        let reference = Color::new(0.2, 0.5, 0.5);

        assert_eq!(reference, a-b);
    }

    #[test]
    fn multiplying_color_by_scalar_computes_correctly() {
        let c = Color::new(0.2, 0.3, 0.4);
        let reference = Color::new(0.4, 0.6, 0.8);

        assert_eq!(reference, c*2.0);
    }

    #[test]
    fn mutiplying_colors_should_compute_correctly() {
        let a = Color::new(1.0, 0.2, 0.4);
        let b = Color::new(0.9, 1.0, 0.1);
        let reference = Color::new(0.9, 0.2, 0.04);

        assert_eq!(reference, a*b);
    }
}