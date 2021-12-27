use super::color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<color::Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas { 
            width, 
            height, 
            pixels: vec![vec![color::Color::default(); width]; height] 
        }
    }

    pub fn write_pixel(
        &mut self, 
        width: usize, 
        height: usize, 
        color: color::Color
    ) -> () {
        self.pixels[width-1][height-1] = color;
    }

    pub fn pixel_at(&self, width: usize, height: usize) -> color::Color {
        self.pixels[width-1][height-1]
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
}

