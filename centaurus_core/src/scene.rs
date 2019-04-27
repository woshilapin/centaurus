use crate::light::Light;
use crate::object::Intersect;
use crate::Camera;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub dimension: u8,
    pub camera: Camera,
    pub background_color: [f64; 3],
    pub lights: Vec<Box<dyn Light>>,
    pub objects: Vec<Box<dyn Intersect>>,
}
