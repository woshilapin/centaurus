use image::color::Color;

pub mod color;

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Vec<Color>>,
}

impl Image {
    pub fn new((w, h): (usize, usize)) -> Image {
        let mut data = Vec::new();
        for i in 0..w {
            let mut line = Vec::new();
            for j in 0..h {
                line.push(Color::new(0, 0, 0));
            }
            data.push(line);
        }
        Image {
            width: w,
            height: h,
            data,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn color(&self, w: usize, h: usize) -> Color {
        self.data[w][h]
    }

    pub fn set_color(&mut self, w: usize, h: usize, color: Color) {
        self.data[w][h] = color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_constructor() {
        let image = Image::new((42, 21));
        assert_eq!(image.width(), 42);
        assert_eq!(image.height(), 21);
        assert_eq!(image.data.len(), 42);
    }
}
