use serde_derive::Deserialize;

#[derive(Debug, Default, Deserialize, Copy, Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub fn get_red(&self) -> u8 {
        self.red
    }

    pub fn get_green(&self) -> u8 {
        self.green
    }

    pub fn get_blue(&self) -> u8 {
        self.blue
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
