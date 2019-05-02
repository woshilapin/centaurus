use crate::object::Intersection;
use crate::Object;
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
impl Object for Sphere {
    /// The intersection between a sphere and a line is documented on Wikipedia
    /// See https://www.wikiwand.com/en/Line%E2%80%93sphere_intersection
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let to_center = ray.origin - self.center;
        let dot_product = ray.direction.dot(&to_center);
        let determinant =
            dot_product * dot_product - (to_center.dot(&to_center) - self.radius * self.radius);
        if determinant < 0.0 {
            None
        } else {
            let root1 = -dot_product - f64::sqrt(determinant);
            let root2 = -dot_product + f64::sqrt(determinant);
            let root = if root1 >= 0.0f64 { root1 } else { root2 };
            let position = ray.origin + root * ray.direction;
            let normal = (position - self.center) / self.radius;
            let intersection = Intersection {
                distance: root,
                position,
                normal,
            };
            Some(intersection)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;
    use pretty_assertions::assert_eq;

    #[test]
    fn no_intersection_with_sphere() {
        let ray = Ray::new(Point3::new(2.0, 2.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let intersection = sphere.intersect(&ray);
        assert_eq!(intersection.is_none(), true);
    }

    #[test]
    fn intersection_with_middle_of_sphere() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let intersection = sphere.intersect(&ray).unwrap();
        assert_eq!(intersection.distance, 0.0);
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
    fn intersection_from_middle_of_sphere() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let intersection = sphere.intersect(&ray).unwrap();
        assert_eq!(intersection.distance, 1.0);
        let p = intersection.position;
        assert_eq!(p[0], 0.0);
        assert_eq!(p[1], 0.0);
        assert_eq!(p[2], 1.0);
        let n = intersection.normal;
        assert_eq!(n[0], 0.0);
        assert_eq!(n[1], 0.0);
        assert_eq!(n[2], 1.0);
    }

    #[test]
    fn intersection_with_tangent_of_sphere() {
        let ray = Ray::new(Point3::new(1.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let intersection = sphere.intersect(&ray).unwrap();
        assert_eq!(intersection.distance, 1.0);
        let p = intersection.position;
        assert_eq!(p[0], 1.0);
        assert_eq!(p[1], 0.0);
        assert_eq!(p[2], 0.0);
        let n = intersection.normal;
        assert_eq!(n[0], 1.0);
        assert_eq!(n[1], 0.0);
        assert_eq!(n[2], 0.0);
    }

    #[test]
    fn intersection_with_normalized_normal() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 3.0);
        let intersection = sphere.intersect(&ray).unwrap();
        assert_eq!(intersection.distance, 2.0);
        let p = intersection.position;
        assert_eq!(p[0], 0.0);
        assert_eq!(p[1], 0.0);
        assert_eq!(p[2], -3.0);
        let n = intersection.normal;
        assert_eq!(n[0], 0.0);
        assert_eq!(n[1], 0.0);
        assert_eq!(n[2], -1.0);
    }
}
