#[macro_use]
extern crate log;

use crate::camera::Camera;
use crate::image::{Color, Image};
use crate::light::{Light, Lightbulb, Spot, Sun};
use crate::object::{Intersect, Triangle};
use crate::ray::Ray;
use indicatif::ProgressBar;
use nalgebra::{Point3, Vector3};
use serde_derive::Deserialize;

mod camera;
pub mod image;
mod light;
mod object;
mod ray;
mod vertex;

#[derive(Debug, Deserialize)]
pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub dimension: u8,
    pub camera: Camera,
    pub background_color: Color,
    pub objects: Vec<Triangle>,
}

impl Scene {
    pub fn render(&self) -> Image {
        let mut image = Image::new((self.width, self.height));
        let _camera = Camera::new(
            Point3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, 0.0, 1.0),
            1.0,
            [1.0, -1.0, -1.0, 1.0],
        );
        let light = Sun::new(Vector3::new(-0.5, 0.5, 1.0));
        let __light = Lightbulb::new(Point3::new(1.0, 1.0, -2.0));
        let _light = Spot::new(
            Point3::new(0.5, 0.5, -1.0),
            Vector3::new(-0.4, -0.4, 1.0),
            0.2,
        );
        let progress_bar = ProgressBar::new(self.height as u64 * self.width as u64);
        for i in 0..self.height as usize {
            for j in 0..self.width as usize {
                progress_bar.inc(1);
                let x = 2.0 * (j as f64) / ((self.width - 1) as f64) - 1.0;
                let y = 2.0 * (i as f64) / ((self.height - 1) as f64) - 1.0;
                let ray = Ray::new(Point3::new(x, y, -1.0), Vector3::new(0.0, 0.0, 1.0));
                image.set_color(i, j, self.background_color);
                for object in &self.objects {
                    if let Some(intersection) = object.intersect(&ray) {
                        let i_position = intersection.position;
                        let i_normal = intersection.normal;
                        if let Some(l_direction) = light.light_direction(&i_position) {
                            let intensity = i_normal.dot(&(-l_direction));
                            if intensity >= 0.0 && intensity <= 1.0 {
                                let intensity = (intensity * f64::from(u8::max_value())) as u8;
                                image.set_color(i, j, Color::new(intensity, intensity, intensity));
                            }
                        }
                    }
                }
            }
        }
        image
    }
}
