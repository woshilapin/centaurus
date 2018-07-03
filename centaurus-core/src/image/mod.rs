use image::color::Color;

mod color;

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Vec<Color>>,
}

impl Image{
    pub fn new((w,h): (usize,usize)) -> Image {
        let mut data = Vec::new();
        for i in 0..w {
            let mut line = Vec::new();
            for j in 0..h {
                line.push(Color::new());
            }
            data.push(line);
        };
        Image {
            width: w,
            height:h,
            data,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_heigth(&self) -> usize {
        self.height
    }

    pub fn get_color(&self, w: usize, h:usize) -> Color {
        self.data[w][h]
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