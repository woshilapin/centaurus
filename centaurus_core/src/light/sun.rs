use crate::light::Light;
use nalgebra::{Point3, Vector3};
use std::option::Option;

pub struct Sun {
    direction: Vector3<f64>,
}

impl Sun {
    pub fn new(direction: Vector3<f64>) -> Sun {
        Sun {
            direction: direction.normalize(),
        }
    }
}

impl Light for Sun {
    fn light_direction(&self, _illuminated_position: &Point3<f64>) -> Option<Vector3<f64>> {
        Some(self.direction)
    }
}
