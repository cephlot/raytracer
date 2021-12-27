pub struct Canvas {
    pub width: u32,
    pub height: u32,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_contain_correct_data() {
        let c = Canvas::new(10, 20);

        assert_eq!(10, c.width);
        assert_eq!(20, c.height)
    }
}

