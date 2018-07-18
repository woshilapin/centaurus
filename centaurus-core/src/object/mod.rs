use nalgebra::{Point3, Vector3};
use std::option::Option;

pub mod triangle;

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        None
    }
}

pub struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Ray {
        Ray {
            origin,
            direction,
        }
    }
    pub fn origin(&self) -> Point3<f64> {
        self.origin
    }
    pub fn direction(&self) -> Vector3<f64> {
        self.direction
    }
}


pub struct Intersection {
    position: Point3<f64>,
}