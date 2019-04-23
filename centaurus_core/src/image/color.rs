use serde_derive::Deserialize;

/// Define a RGB color
#[derive(Debug, Default, Deserialize, Copy, Clone)]
pub struct Color {
    /// Red intensity as an integer from 0 to 256
    pub red: u8,
    /// Green intensity as an integer from 0 to 256
    pub green: u8,
    /// Blue intensity as an integer from 0 to 256
    pub blue: u8,
}

impl Color {
    /// Instantiate a new Color
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_constructor() {
        let color = Color::new(0, 0, 0);
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
    }
}
