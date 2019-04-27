use crate::Scene;
use image::{ImageBuffer, Rgba};

pub trait Renderer {
    fn render(scene: &Scene) -> ImageBuffer<Rgba<f64>, Vec<f64>>;
}

mod default_renderer;

pub use self::default_renderer::*;
