use crate::light::Light;
use image::Rgba;
use nalgebra::{Point3, Vector3};
use std::option::Option;

/// A Spot will light only in a specific direction in a cone of light.
/// It is defined through a position, a direction, and the angle of the cone light.
/// A color is also associated.
pub struct Spot {
    position: Point3<f64>,
    direction: Vector3<f64>,
    cosinus_angle: f64,
    color: Rgba<u8>,
}

impl Spot {
    /// Create a new Spot object.
    /// 
    /// For example, if you want to create a spot, just above the center of the scene, which would light the scene with a green cone of light.
    /// ```
    /// # use centaurus_core::light::Spot;
    /// # use nalgebra::{Point3, Vector3};
    /// # use image::Rgba;
    /// let spot = Spot::new(
    ///     Point3::new(0.0, 1.0, 0.0),
    ///     Vector3::new(0.0, -1.0, 0.0),
    ///     0.5,
    ///     Rgba([0, u8::max_value(), 0, u8::max_value()]),
    /// );
    /// ```
    /// 
    /// Note: the `angle` value is given in radians.
    pub fn new(
        position: Point3<f64>,
        direction: Vector3<f64>,
        angle: f64,
        color: Rgba<u8>,
    ) -> Spot {
        Spot {
            position,
            direction: direction.normalize(),
            cosinus_angle: f64::cos(angle),
            color,
        }
    }
}

impl Light for Spot {
    fn hit(&self, position: &Point3<f64>) -> Option<(Vector3<f64>, Rgba<u8>)> {
        let direction = position - self.position;
        let direction = direction.normalize();
        let cosinus = direction.dot(&self.direction);
        if cosinus > self.cosinus_angle {
            let distance_factor = 1.0f64 / (1.0f64 + direction.norm());
            let color = Rgba([
                ((self.color[0] as f64) * distance_factor) as u8,
                ((self.color[1] as f64) * distance_factor) as u8,
                ((self.color[2] as f64) * distance_factor) as u8,
                self.color[3],
            ]);
            Some((direction, color))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_normalized_direction() {
        let sun = Spot::new(
            Point3::new(-2.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            0.5,
            Rgba([128, 128, 128, u8::max_value()]),
        );
        let hit = sun.hit(&Point3::new(0.0, 0.0, 0.0));
        let (direction, _) = hit.unwrap();
        assert_eq!(direction[0], 1.0);
        assert_eq!(direction[1], 0.0);
        assert_eq!(direction[2], 0.0);
    }

    #[test]
    fn should_return_dimmed_color() {
        let sun = Spot::new(
            Point3::new(-2.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            0.5,
            Rgba([128, 128, 128, u8::max_value()]),
        );
        let hit = sun.hit(&Point3::new(0.0, 0.0, 0.0));
        let (_, color) = hit.unwrap();
        assert_eq!(color[0], 64);
        assert_eq!(color[1], 64);
        assert_eq!(color[2], 64);
        assert_eq!(color[3], u8::max_value());
    }

    #[test]
    fn should_return_none() {
        let sun = Spot::new(
            Point3::new(-2.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            0.5,
            Rgba([128, 128, 128, u8::max_value()]),
        );
        let hit = sun.hit(&Point3::new(-4.0, 0.0, 0.0));
        assert_eq!(hit.is_none(), true);
    }
}
