use crate::light::Light;
use image::Rgba;
use nalgebra::{Point3, Vector3};
use std::option::Option;

pub struct Sun {
    direction: Vector3<f64>,
    color: Rgba<u8>,
}

impl Sun {
    pub fn new(direction: Vector3<f64>, color: Rgba<u8>) -> Sun {
        Sun {
            direction: direction.normalize(),
            color,
        }
    }
}

impl Light for Sun {
    fn hit(&self, _illuminated_position: &Point3<f64>) -> Option<(Vector3<f64>, Rgba<u8>)> {
        Some((self.direction, self.color))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_normalized_direction() {
        let sun = Sun::new(Vector3::new(2.0, 0.0, 0.0), Rgba([12, 34, 56, u8::max_value()]));
        let hit = sun.hit(&Point3::new(0.0, 0.0, 0.0));
        let (direction, _) = hit.unwrap();
        assert_eq!(direction[0], 1.0);
        assert_eq!(direction[1], 0.0);
        assert_eq!(direction[2], 0.0);
    }

    #[test]
    fn should_return_same_color() {
        let sun = Sun::new(Vector3::new(2.0, 0.0, 0.0), Rgba([12, 34, 56, u8::max_value()]));
        let hit = sun.hit(&Point3::new(0.0, 0.0, 0.0));
        let (_, color) = hit.unwrap();
        assert_eq!(color[0], 12);
        assert_eq!(color[1], 34);
        assert_eq!(color[2], 56);
        assert_eq!(color[3], u8::max_value());
    }
}