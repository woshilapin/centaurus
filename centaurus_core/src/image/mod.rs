/// Data matrix to save the colors of the images
pub struct Image {
    /// Width of the image
    pub width: usize,
    /// Height of the image
    pub height: usize,
    /// Data matrix of the colors, with _height_ lines containing each _width_ color cells
    pub data: Vec<Vec<Color>>,
}

impl Image {
    /// Create a new image of size `width`x`height`
    pub fn new((width, height): (usize, usize)) -> Image {
        let mut data = Vec::new();
        for _i in 0..height {
            let mut line = Vec::new();
            for _j in 0..width {
                line.push(Color::new(0, 0, 0));
            }
            data.push(line);
        }
        Image {
            width,
            height,
            data,
        }
    }

    /// Get the color of the pixel at position `width`x`height` 
    pub fn color(&self, width: usize, height: usize) -> Color {
        self.data[height][width]
    }

    /// Change the color of the pixel at position `width`x`height` 
    pub fn set_color(&mut self, width: usize, height: usize, color: Color) {
        self.data[height][width] = color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_constructor() {
        let image = Image::new((42, 21));
        assert_eq!(image.width, 42);
        assert_eq!(image.height, 21);
        assert_eq!(image.data.len(), 42);
    }
}

mod color;

pub use self::color::Color;
