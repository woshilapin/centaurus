use crate::camera::Camera;
use crate::image::{Color, Image};
use crate::light::{Light, Lightbulb, Spot};
use crate::object::{Intersect, Triangle};
use crate::ray::Ray;
use crate::vertex::Vertex;
use nalgebra::{Point3, Vector3};
use serde_derive::Deserialize;

mod camera;
pub mod image;
mod light;
mod object;
mod ray;
mod vertex;

#[derive(Debug, Default, Deserialize, Copy, Clone)]
pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub dimension: u8,
    pub camera: Camera,
}

impl Scene {
    pub fn render(&self) -> Image {
        let mut image = Image::new((self.width, self.height));
        let triangle = Triangle::new([
            Vertex::new(
                Point3::new(-0.5, -0.5, 0.0),
                Vector3::new(-0.5, -0.5, -0.5).normalize(),
            ),
            Vertex::new(
                Point3::new(0.5, -0.5, 0.0),
                Vector3::new(0.0, 0.0, -0.5).normalize(),
            ),
            Vertex::new(
                Point3::new(0.0, 0.5, 0.0),
                Vector3::new(0.5, 0.5, -0.5).normalize(),
            ),
        ]);
        let _camera = Camera::new(
            Point3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, 0.0, 1.0),
            1.0,
            [1.0, -1.0, -1.0, 1.0],
        );
        let _light = Lightbulb::new(Point3::new(0.0, 0.0, -2.0));
        let light = Spot::new(
            Point3::new(0.0, 0.0, -1.0),
            Vector3::new(0.4, -0.4, 1.0),
            0.2,
        );
        for i in 0..self.width as usize {
            for j in 0..self.height as usize {
                let x = (i as f64) * 2.0 / (self.width as f64) - 1.0;
                let y = (j as f64) * 2.0 / (self.height as f64) - 1.0;
                let ray = Ray::new(Point3::new(x, y, -1.0), Vector3::new(0.0, 0.0, 1.0));
                if let Some(intersection) = triangle.intersect(&ray) {
                    let i_position = intersection.position();
                    let i_normal = intersection.normal();
                    if let Some(l_direction) = light.light_direction(&i_position) {
                        let intensity = i_normal.dot(&(-l_direction));
                        let intensity = (intensity * (u8::max_value() as f64)) as u8;
                        image.set_color(i, j, Color::new(intensity, intensity, intensity));
                    }
                }
            }
        }
        image
    }
}