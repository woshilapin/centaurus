
use nalgebra::{Point3, Vector3};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Camera {
    pub position: Point3<f64>,
    pub direction: Vector3<f64>,
    pub focal_length: f64,
    pub upper_bound: f64,
    pub lower_bound: f64,
    pub left_bound: f64,
    pub right_bound: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Point3::new(0f64, 0f64, -1f64),
            direction: Vector3::new(0f64, 0f64, 1f64),
            focal_length: 1f64,
            upper_bound: 0.5,
            lower_bound: -0.5,
            left_bound: -0.5,
            right_bound: 0.5,
        }
    }
}

impl Camera {
    pub fn new(
        position: Point3<f64>,
        direction: Vector3<f64>,
        focal_length: f64,
        bounds: [f64; 4],
    ) -> Camera {
        Camera {
            position,
            direction,
            focal_length,
            upper_bound: bounds[0],
            lower_bound: bounds[1],
            left_bound: bounds[2],
            right_bound: bounds[3],
        }
    }
}
