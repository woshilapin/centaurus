#[macro_use]
extern crate log;

use crate::camera::Camera;

use crate::light::{Light, Lightbulb, Spot, Sun};
use crate::object::Intersect;
use crate::ray::Ray;
use image::{Rgba, RgbaImage};
use indicatif::ProgressBar;
use nalgebra::{Point3, Vector3};
use serde_derive::Deserialize;

pub mod camera;
pub mod light;
pub mod object;
pub mod ray;
pub mod vertex;

#[derive(Deserialize)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub dimension: u8,
    pub camera: Camera,
    pub background_color: [u8; 4],
    pub objects: Vec<Box<dyn Intersect>>,
}

fn bounded_add(v1: u8, v2: u8) -> u8 {
    if (v1 as u16 + v2 as u16) < (u8::max_value() as u16) {
        v1 + v2
    } else {
        u8::max_value()
    }
}
fn bounded_multiply(value: u8, scalar: f64) -> u8 {
    ((value as f64) * scalar) as u8
}
fn multiply_color_scalar(c: Rgba<u8>, scalar: f64) -> Rgba<u8> {
    Rgba([
        bounded_multiply(c[0], scalar),
        bounded_multiply(c[1], scalar),
        bounded_multiply(c[2], scalar),
        c[3],
    ])
}
fn combine_color(c1: Rgba<u8>, c2: Rgba<u8>) -> Rgba<u8> {
    Rgba([
        bounded_add(c1[0], c2[0]),
        bounded_add(c1[1], c2[1]),
        bounded_add(c1[2], c2[2]),
        bounded_add(c1[3], c2[3]),
    ])
}

impl Scene {
    pub fn render(&self) -> RgbaImage {
        let _camera = Camera::new(
            Point3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, 0.0, 1.0),
            1.0,
            [1.0, -1.0, -1.0, 1.0],
        );
        let light1 = Sun::new(
            Vector3::new(0.5, 0.5, 1.0),
            Rgba([128, 128, 128, u8::max_value()]),
        );
        let light2 = Spot::new(
            Point3::new(0.5, 0.5, -2.0),
            Vector3::new(-0.5, -0.5, 1.0),
            0.2,
            Rgba([
                u8::max_value(),
                u8::min_value(),
                u8::min_value(),
                u8::max_value(),
            ]),
        );
        let light3 = Lightbulb::new(
            Point3::new(-1.0, 1.0, 0.0),
            Rgba([
                u8::min_value(),
                u8::max_value(),
                u8::min_value(),
                u8::max_value(),
            ]),
        );
        let lights: Vec<Box<Light>> = vec![Box::new(light1), Box::new(light2), Box::new(light3)];
        let progress_bar = ProgressBar::new(self.height as u64 * self.width as u64);
        RgbaImage::from_fn(self.width, self.height, |i, j| {
            progress_bar.inc(1);
            let x_unit = (self.camera.right_bound - self.camera.left_bound) / ((self.width) as f64);
            let y_unit =
                (self.camera.upper_bound - self.camera.lower_bound) / ((self.height) as f64);
            let x = self.camera.left_bound + (i as f64 + 0.5) * x_unit;
            let y = self.camera.lower_bound + ((self.height - j) as f64 - 0.5) * y_unit;
            let ray = Ray::new(Point3::new(x, y, -1.0), Vector3::new(0.0, 0.0, 1.0));
            let mut color = Rgba(self.background_color);
            for object in &self.objects {
                if let Some(intersection) = object.intersect(&ray) {
                    let i_position = intersection.position;
                    let i_normal = intersection.normal;
                    for light in &lights {
                        if let Some((l_direction, l_color)) = light.hit(&i_position) {
                            let intensity = i_normal.dot(&(-l_direction));
                            if intensity >= 0.0 && intensity <= 1.0 {
                                let new_color = multiply_color_scalar(l_color, intensity);
                                color = combine_color(color, new_color);
                            }
                        }
                    }
                }
            }
            color
        })
    }
}
