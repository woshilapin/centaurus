use image::Rgb;
use nalgebra::{Point3, Vector3};
use std::option::Option;

/// Define functionality for Light objects
#[typetag::deserialize(tag = "type")]
pub trait Light {
    /// Calculate from which direction and with which amount of color the `position` is hit by the current `Light`.
    fn hit(&self, position: &Point3<f64>) -> Option<(Vector3<f64>, Rgb<f64>)>;
}

mod lightbulb;
mod spot;
mod sun;

pub use self::lightbulb::*;
pub use self::spot::*;
pub use self::sun::*;
