extern crate nalgebra;

use camera::Camera;
use image::color::Color;
use image::Image;
use nalgebra::{Point3, Vector3};
use object::triangle::Triangle;
use object::Intersect;
use ray::Ray;

mod camera;
pub mod image;
mod object;
mod ray;

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
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn dimension(&self) -> u8 {
        self.dimension
    }
    pub fn render(&self) -> Image {
        let mut image = Image::new((self.width, self.height));
        let triangle = Triangle::new(
            [
                Point3::new(-0.5, -0.5, 0.0),
                Point3::new(0.5, -0.5, 0.0),
                Point3::new(0.0, 0.5, 0.0),
            ],
            Vector3::new(0.0, 0.0, -1.0),
        );
        let camera = Camera::new(
            Point3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, 0.0, 1.0),
            1.0,
            [1.0, -1.0, -1.0, 1.0],
        );
        for i in 0..self.width() as usize {
            for j in 0..self.height() as usize {
                let x = (i as f64) * 2.0 / (self.width() as f64) - 1.0;
                let y = (j as f64) * 2.0 / (self.height() as f64) - 1.0;
                let ray = Ray::new(Point3::new(x, y, -1.0), Vector3::new(0.0, 0.0, 1.0));
                match triangle.intersect(&ray) {
                    Some(intersection) => image.set_color(
                        i,
                        j,
                        Color::new(u8::max_value(), u8::max_value(), u8::max_value()),
                    ),
                    None => image.set_color(i, j, Color::new(0, 0, 0)),
                }
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_constructor() {
        let scene: Scene = SceneBuilder::new().build();
        assert_eq!(scene.dimension(), 3);
        assert_eq!(scene.width(), 600);
        assert_eq!(scene.height(), 400);
    }
}
