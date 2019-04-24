use crate::light::Light;
use image::Rgba;
use nalgebra::{Point3, Vector3};
use std::option::Option;

pub struct Lightbulb {
    position: Point3<f64>,
    color: Rgba<u8>,
}

impl Lightbulb {
    pub fn new(position: Point3<f64>, color: Rgba<u8>) -> Lightbulb {
        Lightbulb { position, color }
    }
}

impl Light for Lightbulb {
    fn hit(&self, illuminated_position: &Point3<f64>) -> Option<(Vector3<f64>, Rgba<u8>)> {
        let direction = illuminated_position - self.position;
        let direction = direction.normalize();
        let distance_factor = 1.0f64 / (1.0f64 + direction.norm());
        let color = Rgba([
            ((self.color[0] as f64) * distance_factor) as u8,
            ((self.color[1] as f64) * distance_factor) as u8,
            ((self.color[2] as f64) * distance_factor) as u8,
            self.color[3],
        ]);
        Some((direction, color))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_normalized_direction() {
        let sun = Lightbulb::new(Point3::new(-2.0, 0.0, 0.0), Rgba([u8::max_value(), u8::max_value(), u8::max_value(), u8::max_value()]));
        let hit = sun.hit(&Point3::new(0.0, 0.0, 0.0));
        let (direction, _) = hit.unwrap();
        assert_eq!(direction[0], 1.0);
        assert_eq!(direction[1], 0.0);
        assert_eq!(direction[2], 0.0);
    }

    #[test]
    fn should_return_dimmed_color() {
        let sun = Lightbulb::new(Point3::new(2.0, 0.0, 0.0), Rgba([128, 128, 128, u8::max_value()]));
        let hit = sun.hit(&Point3::new(0.0, 0.0, 0.0));
        let (_, color) = hit.unwrap();
        assert_eq!(color[0], 64);
        assert_eq!(color[1], 64);
        assert_eq!(color[2], 64);
        assert_eq!(color[3], u8::max_value());
    }
}