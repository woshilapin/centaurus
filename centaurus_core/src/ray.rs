use nalgebra::{Point3, Vector3};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }
}
