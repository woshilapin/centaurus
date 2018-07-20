use nalgebra::{Point3, Vector3};
use ray::Ray;
use std::option::Option;

pub mod triangle;

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        None
    }
}

pub struct Intersection {
    position: Point3<f64>,
}
