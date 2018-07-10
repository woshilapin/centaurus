#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use image::Image;
use camera::Camera;

pub mod image;
mod camera;
mod object;

#[derive(Debug, Default, Copy, Clone)]
pub struct Scene {
    width: usize,
    height: usize,
    dimension: u8,
}

#[derive(Default)]
pub struct SceneBuilder {
    scene: Scene,
}

impl SceneBuilder {
    pub fn new() -> SceneBuilder {
        SceneBuilder {
            scene: Scene {
                width: 600,
                height: 400,
                dimension: 3,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn with_width(&mut self, width: usize) -> &mut Self {
        self.scene.width = width;
        self
    }

    pub fn with_height(&mut self, height: usize) -> &mut Self {
        self.scene.height = height;
        self
    }

    pub fn with_dimension(&mut self, dimension: u8) -> &mut Self {
        self.scene.dimension = dimension;
        self
    }

    pub fn build(&self) -> Scene {
        self.scene.clone()
    }
}

impl Scene {
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_dimension(&self) -> u8 {
        self.dimension
    }
    pub fn render(&self) -> Image {
        Image::new((self.width, self.height))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_constructor() {
        let scene: Scene = SceneBuilder::new().build();
        assert_eq!(scene.get_dimension(), 3);
        assert_eq!(scene.get_width(), 600);
        assert_eq!(scene.get_height(), 400);
    }
}
