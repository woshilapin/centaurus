use crate::Camera;
use crate::Light;
use crate::Object;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub dimension: u8,
    pub camera: Camera,
    pub background_color: [f64; 3],
    pub lights: Vec<Box<dyn Light>>,
    pub objects: Vec<Box<dyn Object>>,
}
