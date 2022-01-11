use crate::graphics::Color;
use crate::math::Tuple;

/// Representation of a point light - a light source with no size
#[derive(Debug, Copy, Clone)]
pub struct Light {
    /// Position of the point light
    pub position: Tuple,
    /// Brightness of the light
    pub intensity: Color,
    _private: (),
}

impl Light {
    /// Creates a new point light given a position and an intensity
    ///
    /// # Arguments
    ///
    /// * `position` - position of the point light
    /// * `intensity` - brightness of the point light
    pub fn new(position: Tuple, intensity: Color) -> Light {
        Light {
            position,
            intensity,
            _private: (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_contain_correct_data() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Tuple::point(0.0, 0.0, 0.0);

        let light = Light::new(position, intensity);

        assert_eq!(intensity, light.intensity);
        assert_eq!(position, light.position);
    }
}
