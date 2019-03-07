use crate::ray::Ray;
use nalgebra::{Point3, Vector3};
use std::option::Option;

pub trait Intersect {
    fn intersect(&self, _ray: &Ray) -> Option<Intersection> {
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

mod triangle;

pub use self::triangle::*;
