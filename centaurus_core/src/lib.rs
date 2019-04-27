#[macro_use]
extern crate log;

mod camera;
mod ray;
mod scene;
mod serde;
mod vertex;

pub mod light;
pub mod object;
pub mod render;

pub use self::camera::Camera;
pub use self::ray::Ray;
pub use self::scene::Scene;
pub use self::vertex::Vertex;
