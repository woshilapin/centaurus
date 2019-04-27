use crate::light::Light;
use image::Rgb;
use nalgebra::{Point3, Vector3};
use serde_derive::Deserialize;
use std::option::Option;

/// A Lightbulb will light the whole scene from one point in space.
/// Color of the light will dim inversely proportional to the distance to the surface of impact.
#[derive(Deserialize)]
pub struct Lightbulb {
    position: Point3<f64>,
    #[serde(deserialize_with = "crate::serde::deserialize_rgb")]
    color: Rgb<u8>,
}

impl Lightbulb {
    /// Create a new Lightbulb object.
    ///
    /// For example, you can create a red lightbulb top-right of the center of the scene with the following code.
    /// ```
    /// # use centaurus_core::light::Lightbulb;
    /// # use nalgebra::Point3;
    /// # use image::Rgb;
    /// let red_lightbulb = Lightbulb::new(
    ///     Point3::new(1.0, 1.0, 0.0),
    ///     Rgb([u8::max_value(), 0, 0]),
    /// );
    /// ```
    pub fn new(position: Point3<f64>, color: Rgb<u8>) -> Lightbulb {
        Lightbulb { position, color }
    }
}

#[typetag::deserialize(name = "lightbulb")]
impl Light for Lightbulb {
    fn hit(&self, position: &Point3<f64>) -> Option<(Vector3<f64>, Rgb<u8>)> {
        let direction = position - self.position;
        let direction = direction.normalize();
        let distance_factor = 1.0f64 / (1.0f64 + direction.norm());
        let color = Rgb([
            ((self.color[0] as f64) * distance_factor) as u8,
            ((self.color[1] as f64) * distance_factor) as u8,
            ((self.color[2] as f64) * distance_factor) as u8,
        ]);
        Some((direction, color))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_normalized_direction() {
        let sun = Lightbulb::new(
            Point3::new(-2.0, 0.0, 0.0),
            Rgb([u8::max_value(), u8::max_value(), u8::max_value()]),
        );
        let hit = sun.hit(&Point3::new(0.0, 0.0, 0.0));
        let (direction, _) = hit.unwrap();
        assert_eq!(direction[0], 1.0);
        assert_eq!(direction[1], 0.0);
        assert_eq!(direction[2], 0.0);
    }

    #[test]
    fn should_return_dimmed_color() {
        let sun = Lightbulb::new(Point3::new(2.0, 0.0, 0.0), Rgb([128, 128, 128]));
        let hit = sun.hit(&Point3::new(0.0, 0.0, 0.0));
        let (_, color) = hit.unwrap();
        assert_eq!(color[0], 64);
        assert_eq!(color[1], 64);
        assert_eq!(color[2], 64);
        assert_eq!(color[3], u8::max_value());
    }
}
