use image::Rgba;
use nalgebra::{Point3, Vector3};
use std::option::Option;

/// Define functionality for Light objects
pub trait Light {
    /// Calculate from which direction and with which amount of color the `position` is hit by the current `Light`.
    fn hit(&self, position: &Point3<f64>) -> Option<(Vector3<f64>, Rgba<u8>)>;
}

mod lightbulb;
mod spot;
mod sun;

pub use self::lightbulb::*;
pub use self::spot::*;
pub use self::sun::*;
