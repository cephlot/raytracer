use super::color;

/// Represents a two-dimensional grid of pixels
#[derive(Debug, Clone)]
pub struct Canvas {
    /// Width of the canvas
    pub width: usize,
    /// Height of the canvas
    pub height: usize,
    pixels: Vec<Vec<color::Color>>,
}

impl Canvas {
    /// Creates a new canvas, given a width and a height
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas { 
            width, 
            height, 
            pixels: vec![vec![color::Color::default(); width]; height] 
        }
    }

    /// Writes a pixel to the canvas at a point with the given color
    pub fn write_pixel(
        &mut self, 
        width: usize, 
        height: usize, 
        color: color::Color
    ) -> () {
        self.pixels[height][width] = color;
    }

    /// Returns the color of the pixel on the canvas at the given point
    pub fn pixel_at(&self, width: usize, height: usize) -> color::Color {
        self.pixels[height][width]
    }

    /// Converts the canvas to a PPM-encoded string
    pub fn to_ppm(&self) -> String {
        let mut s = String::from(
            format!("P3\n{} {}\n255", self.width, self.height)
        );

        for i in &self.pixels {
            s.push('\n');

            let mut z = String::new();

            for j in i {
                let st = &format!("{}", clamp(j.r)).to_owned();
                
                if z.len() + st.len() >=70 {
                    z.remove(z.len()-1);
                    z.push('\n');
                    s.push_str(&z.to_owned());
                    z = String::new();
                }

                z.push_str(&format!("{} ", st));

                let st = &format!("{}", clamp(j.g)).to_owned();

                if z.len() + st.len() >=70 {
                    z.remove(z.len()-1);
                    z.push('\n');
                    s.push_str(&z.to_owned());
                    z = String::new();
                }
                
                z.push_str(&format!("{} ", st));

                let st = &format!("{}", clamp(j.b)).to_owned();

                if z.len() + st.len() >=70 {
                    z.remove(z.len()-1);
                    z.push('\n');
                    s.push_str(&z.to_owned());
                    z = String::new();
                }
                
                z.push_str(&format!("{} ", st));
            }

            z.remove(z.len()-1);
            s.push_str(&z.to_owned());
        }
        
        s.push('\n');
        s
    }
}

fn clamp(v: f64) -> u8 {
    if v < 0.0 {
        0
    } else if v > 1.0 {
        255
    } else {
        (v*(255 as f64)).ceil() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::color::Color;

    #[test]
    fn should_contain_correct_data() {
        let c = Canvas::new(10, 20);

        assert_eq!(10, c.width);
        assert_eq!(20, c.height)
    }

    #[test]
    fn should_write_correct_pixel() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);

        assert_eq!(red, c.pixel_at(2, 3));
    }

    #[test]
    fn should_construct_ppm_header() {
        let c = Canvas::new(5, 3);

        assert_eq!(true, c.to_ppm().contains("P3\n5 3\n255"));
    }

    #[test]
    fn should_construct_ppm_pixel_data_correctly() {
        let mut d = Canvas::new(5, 3);
        let a = color::Color::new(1.5, 0.0, 0.0);
        let b = color::Color::new(0.0, 0.5, 0.0);
        let c = color::Color::new(-0.5, 0.0, 1.0);

        d.write_pixel(0, 0, a);
        d.write_pixel(2, 1, b);
        d.write_pixel(4, 2, c);

        assert_eq!(true, d.to_ppm().contains("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"));
    }

    #[test]
    fn should_not_be_longer_than_70_characters() {
        let mut c = Canvas::new(10, 2);

        for i in 0..10 {
            for j in 0..2 {
                c.write_pixel(i, j, color::Color::new(1.0, 0.8, 0.6));
            }
        }

        eprintln!("{}", c.to_ppm());

        assert_eq!(true, c.to_ppm().contains("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153"));
    }

    #[test]
    fn should_contain_newline_as_last_character() {
        let c = Canvas::new(5, 3);

        assert_eq!('\n', c.to_ppm().chars().last().unwrap());
    }
}

