use crate::Ray;
use nalgebra::{Point3, Vector3};
use std::option::Option;

#[typetag::deserialize(tag = "type")]
pub trait Object {
    fn intersect(&self, _ray: &Ray) -> Option<Intersection> {
        None
    }
}

#[derive(Debug)]
pub struct Intersection {
    pub distance: f64,
    pub position: Point3<f64>,
    pub normal: Vector3<f64>,
}

mod sphere;
mod triangle;

pub use self::sphere::Sphere;
pub use self::triangle::Triangle;
