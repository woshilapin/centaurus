use nalgebra::{Point3, Vector3};
use ray::Ray;
use std::option::Option;

pub mod triangle;

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        None
    }
}

#[derive(Debug)]
pub struct Intersection {
    position: Point3<f64>,
    normal: Vector3<f64>,
}

impl Intersection {
    pub fn position(&self) -> Point3<f64> {
        self.position
    }
    pub fn normal(&self) -> Vector3<f64> {
        self.normal
    }
}
