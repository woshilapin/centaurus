use nalgebra::{Point3, Scalar, Vector3};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Vertex<T>
where
    T: Scalar,
{
    position: Point3<T>,
    normal: Vector3<T>,
}

impl<T> Vertex<T>
where
    T: Scalar,
{
    pub fn new(position: Point3<T>, normal: Vector3<T>) -> Vertex<T> {
        Vertex { position, normal }
    }
    pub fn position(&self) -> Point3<T> {
        self.position
    }
    pub fn normal(&self) -> Vector3<T> {
        self.normal
    }
}
