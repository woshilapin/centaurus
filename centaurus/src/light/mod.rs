use nalgebra::{Point3, Vector3};
use std::option::Option;

pub trait Light {
    fn light_direction(&self, illuminated_position: &Point3<f64>) -> Option<Vector3<f64>>;
}

mod sun;
mod lightbulb;
mod spot;

pub use self::sun::*;
pub use self::lightbulb::*;
pub use self::spot::*;
