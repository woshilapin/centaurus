use nalgebra::{Matrix, Matrix4, MatrixArray, U4, Vector3, Vector4};
use std::option::Option;

pub mod triangle;

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        None
    }
}

pub struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        Ray {
            origin,
            direction,
        }
    }
    pub fn origin(&self) -> Vector3<f64> {
        self.origin
    }
    pub fn direction(&self) -> Vector3<f64> {
        self.direction
    }
}


pub struct Intersection {
    position: Vector3<f64>,
}