use crate::light::Light;
use nalgebra::{Point3, Vector3};
use std::option::Option;

pub struct Lightbulb {
    position: Point3<f64>,
}

impl Lightbulb {
    pub fn new(position: Point3<f64>) -> Lightbulb {
        Lightbulb {
            position,
        }
    }
}

impl Light for Lightbulb {
    fn light_direction(&self, illuminated_position: &Point3<f64>) -> Option<Vector3<f64>> {
        let direction = illuminated_position - self.position;
        Some(direction.normalize())
    }
}
