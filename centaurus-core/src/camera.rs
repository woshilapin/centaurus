use nalgebra::{Point3, Vector3};

pub struct Camera {
    position: Point3<f64>,
    direction: Vector3<f64>,
    focal_length: f64,
    upper_bound: f64,
    lower_bound: f64,
    left_bound: f64,
    right_bound: f64,
}

impl Camera {
    pub fn new(position: Point3<f64>, direction: Vector3<f64>, focal_length: f64, bounds: [f64; 4]) -> Camera {
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