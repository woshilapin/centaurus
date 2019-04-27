use crate::object::{Intersect, Intersection};
use crate::Ray;
use nalgebra::Point3;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

#[typetag::deserialize(name = "sphere")]
impl Intersect for Sphere {
    /// The intersection between a sphere and a line is documented on Wikipedia
    /// See https://www.wikiwand.com/en/Line%E2%80%93sphere_intersection
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let o_minus_c = ray.origin - self.center;
        let l_o_minus_c = ray.direction.dot(&o_minus_c);
        let determinant =
            l_o_minus_c * l_o_minus_c - (o_minus_c.norm() - self.radius * self.radius);
        if determinant < 0.0 {
            None
        } else {
            let d1 = -l_o_minus_c - f64::sqrt(determinant);
            let d2 = -l_o_minus_c + f64::sqrt(determinant);
            let d = if d1 > d2 { d2 } else { d1 };
            let position = ray.origin + d * ray.direction;
            let normal = position - self.center;
            let intersection = Intersection { position, normal };
            Some(intersection)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;

    #[test]
    fn no_intersection_with_sphere() {
        let ray = Ray::new(Point3::new(2.0, 2.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let intersection = sphere.intersect(&ray);
        assert_eq!(intersection.is_none(), true);
    }

    #[test]
    fn intersection_with_middle_of_sphere() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let intersection = sphere.intersect(&ray).unwrap();
        let p = intersection.position;
        assert_eq!(p[0], 0.0);
        assert_eq!(p[1], 0.0);
        assert_eq!(p[2], -1.0);
        let n = intersection.normal;
        assert_eq!(n[0], 0.0);
        assert_eq!(n[1], 0.0);
        assert_eq!(n[2], -1.0);
    }

    #[test]
    fn intersection_with_tangent_of_sphere() {
        let ray = Ray::new(Point3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let intersection = sphere.intersect(&ray).unwrap();
        let p = intersection.position;
        assert_eq!(p[0], 1.0);
        assert_eq!(p[1], 0.0);
        assert_eq!(p[2], 0.0);
        let n = intersection.normal;
        assert_eq!(n[0], 1.0);
        assert_eq!(n[1], 0.0);
        assert_eq!(n[2], 0.0);
    }
}
