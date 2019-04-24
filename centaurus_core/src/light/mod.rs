use image::Rgba;
use nalgebra::{Point3, Vector3};
use std::option::Option;

pub trait Light {
    fn hit(&self, illuminated_position: &Point3<f64>) -> Option<(Vector3<f64>, Rgba<u8>)>;
}

mod lightbulb;
mod spot;
mod sun;

pub use self::lightbulb::*;
pub use self::spot::*;
pub use self::sun::*;
