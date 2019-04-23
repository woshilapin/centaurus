use alga::general::Real;
use nalgebra::{Point3, Scalar, Vector3};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Vertex<T>
where
    T: Real + Scalar,
{
    pub position: Point3<T>,
    pub normal: Vector3<T>,
}

impl<T> Vertex<T>
where
    T: Real + Scalar,
{
    pub fn new(position: Point3<T>, normal: Vector3<T>) -> Vertex<T> {
        Vertex {
            position,
            normal: normal.normalize(),
        }
    }
}
