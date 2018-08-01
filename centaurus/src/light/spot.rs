use light::Light;
use nalgebra::{Point3, Vector3};
use std::option::Option;

pub struct Spot {
    position: Point3<f64>,
    direction: Vector3<f64>,
    angle: f64,
    cosinus_angle: f64,
}

impl Spot {
    pub fn new(position: Point3<f64>, direction: Vector3<f64>, angle: f64) -> Spot {
        Spot {
            position,
            direction: direction.normalize(),
            angle,
            cosinus_angle: f64::cos(angle),
        }
    }
}

impl Light for Spot {
    fn light_direction(&self, illuminated_position: &Point3<f64>) -> Option<Vector3<f64>> {
        let direction = illuminated_position - self.position;
        let direction = direction.normalize();
        let cosinus = direction.dot(&self.direction);
        if cosinus > self.cosinus_angle {
            Some(direction)
        } else {
            None
        }
    }
}
