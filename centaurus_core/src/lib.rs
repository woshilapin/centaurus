#[macro_use]
extern crate log;

use crate::camera::Camera;

use crate::light::Light;
use crate::object::Intersect;
use crate::ray::Ray;
use image::{ImageBuffer, Rgba};
use indicatif::ProgressBar;
use nalgebra::{Point3, Vector3};
use serde_derive::Deserialize;

pub mod camera;
pub mod light;
pub mod object;
pub mod ray;
mod serde;
pub mod vertex;

#[derive(Deserialize)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub dimension: u8,
    pub camera: Camera,
    pub background_color: [f64; 3],
    pub lights: Vec<Box<dyn Light>>,
    pub objects: Vec<Box<dyn Intersect>>,
}

fn bounded_add(v1: f64, v2: f64) -> f64 {
    if v1 + v2 <= 1.0f64 {
        v1 + v2
    } else {
        1.0f64
    }
}
fn bounded_multiply(value: f64, scalar: f64) -> f64 {
    value * scalar
}
fn multiply_color_scalar(c: Rgba<f64>, scalar: f64) -> Rgba<f64> {
    Rgba([
        bounded_multiply(c[0], scalar),
        bounded_multiply(c[1], scalar),
        bounded_multiply(c[2], scalar),
        c[3],
    ])
}
fn combine_color(c1: Rgba<f64>, c2: Rgba<f64>) -> Rgba<f64> {
    Rgba([
        bounded_add(c1[0], c2[0]),
        bounded_add(c1[1], c2[1]),
        bounded_add(c1[2], c2[2]),
        bounded_add(c1[3], c2[3]),
    ])
}

impl Scene {
    pub fn render(&self) -> ImageBuffer<Rgba<f64>, Vec<f64>> {
        let progress_bar = ProgressBar::new(u64::from(self.height) * u64::from(self.width));
        ImageBuffer::from_fn(self.width, self.height, |i, j| {
            progress_bar.inc(1);
            let x_unit = (self.camera.right_bound - self.camera.left_bound) / f64::from(self.width);
            let y_unit =
                (self.camera.upper_bound - self.camera.lower_bound) / f64::from(self.height);
            let x = self.camera.left_bound + (f64::from(i) + 0.5f64) * x_unit;
            let y = self.camera.lower_bound + (f64::from(self.height - j) - 0.5f64) * y_unit;
            let ray = Ray::new(Point3::new(x, y, -1.0), Vector3::new(0.0, 0.0, 1.0));
            let mut color = Rgba([
                self.background_color[0],
                self.background_color[1],
                self.background_color[2],
                1.0f64,
            ]);
            for object in &self.objects {
                if let Some(intersection) = object.intersect(&ray) {
                    let i_position = intersection.position;
                    let i_normal = intersection.normal;
                    for light in &self.lights {
                        if let Some((l_direction, l_color)) = light.hit(&i_position) {
                            let intensity = i_normal.dot(&(-l_direction));
                            if intensity >= 0.0 && intensity <= 1.0 {
                                let l_color = Rgba([l_color[0], l_color[1], l_color[2], 1.0f64]);
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
