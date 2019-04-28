use crate::Light;
use image::Rgb;
use nalgebra::{Point3, Vector3};
use serde_derive::Deserialize;
use std::option::Option;

/// A sun will light in one direction without dimming with distance.
#[derive(Deserialize)]
pub struct Sun {
    direction: Vector3<f64>,
    #[serde(deserialize_with = "crate::serde::deserialize_rgb")]
    color: Rgb<f64>,
}

impl Sun {
    ///
    ///
    /// For example, creating a blue light that would shine from the zenith.
    /// ```
    /// # use centaurus_core::light::Sun;
    /// # use nalgebra::Vector3;
    /// # use image::Rgb;
    /// let sun = Sun::new(
    ///     Vector3::new(0.0, -1.0, 0.0),
    ///     Rgb([0.0, 0.0, 1.0])
    /// );
    /// ```
    pub fn new(direction: Vector3<f64>, color: Rgb<f64>) -> Sun {
        Sun {
            direction: direction.normalize(),
            color,
        }
    }
}

#[typetag::deserialize(name = "sun")]
impl Light for Sun {
    fn hit(&self, _position: &Point3<f64>) -> Option<(Vector3<f64>, Rgb<f64>)> {
        Some((self.direction, self.color))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_return_normalized_direction() {
        let sun = Sun::new(Vector3::new(2.0, 0.0, 0.0), Rgb([0.5, 0.5, 0.5]));
        let hit = sun.hit(&Point3::new(0.0, 0.0, 0.0));
        let (direction, _) = hit.unwrap();
        assert_eq!(direction[0], 1.0);
        assert_eq!(direction[1], 0.0);
        assert_eq!(direction[2], 0.0);
    }

    #[test]
    fn should_return_same_color() {
        let sun = Sun::new(Vector3::new(2.0, 0.0, 0.0), Rgb([0.5, 0.5, 0.5]));
        let hit = sun.hit(&Point3::new(0.0, 0.0, 0.0));
        let (_, color) = hit.unwrap();
        assert_eq!(color[0], 0.5);
        assert_eq!(color[1], 0.5);
        assert_eq!(color[2], 0.5);
    }
}
