use crate::graphics::{Color, Light};
use crate::math::{reflect, Tuple};

/// Representation of a surface material
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    /// Surface color
    color: Color,
    /// Ambient reflection, i.e. background lighting
    ambient: f64,
    /// Diffuse reflection - light reflected from a matte surface
    diffuse: f64,
    /// Specular reflection - reflection of the light source itself
    specular: f64,
    /// The higher this valye, the smaller and tighter the specular highlight
    shininess: f64,
}

impl Material {
    /// Creates a new material with default values
    pub fn new() -> Material {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    /// Shades the material given a point light, position on the object, an
    /// observing point and a normal vector
    ///
    /// # Arguments
    ///
    /// * `light` - point light illuminating the object
    /// * `position` - position to shade
    /// * `eye` - observing point
    /// * `normal` - normal vector
    pub fn lighting(&self, light: Light, position: Tuple, eye: Tuple, normal: Tuple) -> Color {
        let color = self.color * light.intensity;
        let light_v = (light.position - position).normalize();
        let dot = Tuple::dot(&light_v, &normal);
        let ambient = color * self.ambient;
        let mut diffuse = Color::new(0.0, 0.0, 0.0);
        let mut specular = Color::new(0.0, 0.0, 0.0);

        if dot < 0.0 {
            diffuse = Color::new(0.0, 0.0, 0.0);
            specular = Color::new(0.0, 0.0, 0.0);
        } else {
            diffuse = color * self.diffuse * dot;
            let reflect = reflect(-light_v, normal);
            let dot_eye = Tuple::dot(&reflect, &eye);

            if dot_eye <= 0.0 {
                specular = Color::new(0.0, 0.0, 0.0);
            } else {
                let factor = dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_contain_correct_data() {
        let m = Material::new();

        assert_eq!(Color::new(1.0, 1.0, 1.0), m.color);
        assert_eq!(0.1, m.ambient);
        assert_eq!(0.9, m.diffuse);
        assert_eq!(0.9, m.specular);
        assert_eq!(200.0, m.shininess);
    }

    #[test]
    fn should_calculate_lighting_correctly() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let lighting = m.lighting(light, position, eye, normal);

        assert_eq!(Color::new(1.9, 1.9, 1.9), lighting);
    }

    #[test]
    fn should_calculate_lighting_correctly_with_different_eye_angle() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let lighting = m.lighting(light, position, eye, normal);

        assert_eq!(Color::new(1.0, 1.0, 1.0), lighting);
    }

    #[test]
    fn should_calculate_lighting_correctly_with_different_light_angle() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let lighting = m.lighting(light, position, eye, normal);

        assert_eq!(Color::new(0.7364, 0.7364, 0.7364), lighting);
    }

    #[test]
    fn should_calculate_lighting_correctly_with_different_angles() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::point(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let lighting = m.lighting(light, position, eye, normal);

        assert_eq!(Color::new(1.6364, 1.6364, 1.6364), lighting);
    }

    #[test]
    fn should_calculate_lighting_correctly_with_light_behind_object() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let lighting = m.lighting(light, position, eye, normal);

        assert_eq!(Color::new(0.1, 0.1, 0.1), lighting);
    }
}
