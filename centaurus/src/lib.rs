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
    pub background_color: Color,
}

impl Scene {
    pub fn render(&self) -> Image {
        let mut image = Image::new((self.width, self.height));
        let triangle1 = Triangle::new([
            Vertex::new(
                Point3::new(0.0, 0.0, -0.5),
                Vector3::new(0.0, 0.0, -1.0).normalize(),
            ),
            Vertex::new(
                Point3::new(-0.5, 0.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0).normalize(),
            ),
            Vertex::new(
                Point3::new(0.0, 0.5, 0.0),
                Vector3::new(0.0, 1.0, 0.0).normalize(),
            ),
        ]);
        let triangle2 = Triangle::new([
            Vertex::new(
                Point3::new(0.0, 0.0, -0.5),
                Vector3::new(0.0, 0.0, -1.0).normalize(),
            ),
            Vertex::new(
                Point3::new(0.5, 0.0, 0.0),
                Vector3::new(1.0, 0.0, 0.0).normalize(),
            ),
            Vertex::new(
                Point3::new(0.0, 0.5, 0.0),
                Vector3::new(0.0, 1.0, 0.0).normalize(),
            ),
        ]);
        let triangle3 = Triangle::new([
            Vertex::new(
                Point3::new(0.0, 0.0, -0.5),
                Vector3::new(0.0, 0.0, -1.0).normalize(),
            ),
            Vertex::new(
                Point3::new(-0.5, 0.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0).normalize(),
            ),
            Vertex::new(
                Point3::new(0.0, -0.5, 0.0),
                Vector3::new(0.0, -1.0, 0.0).normalize(),
            ),
        ]);
        let triangle4 = Triangle::new([
            Vertex::new(
                Point3::new(0.0, 0.0, -0.5),
                Vector3::new(0.0, 0.0, -1.0).normalize(),
            ),
            Vertex::new(
                Point3::new(0.5, 0.0, 0.0),
                Vector3::new(1.0, 0.0, 0.0).normalize(),
            ),
            Vertex::new(
                Point3::new(0.0, -0.5, 0.0),
                Vector3::new(0.0, -1.0, 0.0).normalize(),
            ),
        ]);
        let objects = vec![triangle1, triangle2, triangle3, triangle4];
        let _camera = Camera::new(
            Point3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, 0.0, 1.0),
            1.0,
            [1.0, -1.0, -1.0, 1.0],
        );
        let light = Lightbulb::new(Point3::new(2.0, 2.0, -2.0));
        let _light = Spot::new(
            Point3::new(0.0, 0.0, -1.0),
            Vector3::new(0.4, -0.4, 1.0),
            0.2,
        );
        for i in 0..self.width as usize {
            for j in 0..self.height as usize {
                let x = (i as f64) / ((self.width - 1) as f64) - 0.5;
                let y = (j as f64) / ((self.height - 1) as f64) - 0.5;
                let ray = Ray::new(Point3::new(x, y, -1.0), Vector3::new(0.0, 0.0, 1.0));
                image.set_color(i, j, self.background_color);
                for object in &objects {
                    if let Some(intersection) = object.intersect(&ray) {
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
        }
        image
    }
}