use nalgebra::{Matrix4, Point3, Vector3};
use object::Intersect;
use object::Intersection;
use ray::Ray;
use std::option::Option;

pub struct Triangle {
    vertices: [Point3<f64>; 3],
    normal: Vector3<f64>,
}

impl Triangle {
    pub fn new(vertices: [Point3<f64>; 3], normal: Vector3<f64>) -> Triangle {
        Triangle { vertices, normal }
    }
}

impl Intersect for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let origin = ray.origin().to_homogeneous();
        let direction = ray.direction().to_homogeneous();
        let edge_1 = self.vertices[1] - self.vertices[0];
        let edge_2 = self.vertices[2] - self.vertices[0];
        let affine_matrix = Matrix4::from_columns(&[
            edge_1.to_homogeneous(),
            edge_2.to_homogeneous(),
            self.normal.to_homogeneous(),
            self.vertices[0].to_homogeneous(),
        ]);
        let inverse_affine_matrix = match affine_matrix.try_inverse() {
            Some(i) => i,
            None => return None,
        };
        let new_origin = inverse_affine_matrix * origin;
        let new_direction = inverse_affine_matrix * direction;
        let t = -new_origin[2] / new_direction[2];
        let u = new_origin[0] + t * new_direction[0];
        let v = new_origin[1] + t * new_direction[1];
        if t >= 0.0 && u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
            let intersection = Intersection {
                position: ray.origin() + t * ray.direction(),
                normal: self.normal,
            };
            Some(intersection)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_intersection_with_perpendicular_plan() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
        let triangle = Triangle {
            vertices: [
                Point3::new(-1.0, 1.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
                Point3::new(0.0, 1.0, 1.0),
            ],
            normal: Vector3::new(0.0, 0.0, -1.0),
        };
        let intersection = triangle.intersect(&ray);
        assert_eq!(intersection.is_none(), true);
    }

    #[test]
    fn intersection_outside_triangle() {
        let ray = Ray::new(Point3::new(2.0, 2.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
        let triangle = Triangle {
            vertices: [
                Point3::new(-1.0, 1.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
                Point3::new(1.0, -1.0, 0.0),
            ],
            normal: Vector3::new(0.0, 0.0, -1.0),
        };
        let intersection = triangle.intersect(&ray);
        assert_eq!(intersection.is_none(), true);
    }

    #[test]
    fn intersection_inside_symetric_triangle() {
        let ray = Ray::new(Point3::new(-1.0, -1.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
        let triangle = Triangle {
            vertices: [
                Point3::new(-1.0, 1.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
                Point3::new(1.0, -1.0, 0.0),
            ],
            normal: Vector3::new(0.0, 0.0, -1.0),
        };
        let intersection = triangle.intersect(&ray);
        assert_eq!(intersection.is_none(), true);
    }

    #[test]
    fn intersection_with_triangle() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
        let triangle = Triangle {
            vertices: [
                Point3::new(-1.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
            ],
            normal: Vector3::new(0.0, 0.0, -1.0),
        };
        let intersection_opt = triangle.intersect(&ray);
        assert_eq!(intersection_opt.is_some(), true);
        let intersection = intersection_opt.unwrap();
        let position = intersection.position;
        assert_eq!(position[0], 0.0);
        assert_eq!(position[1], 0.0);
        assert_eq!(position[2], 0.0);
        let normal = intersection.normal;
        assert_eq!(normal[0], 0.0);
        assert_eq!(normal[1], 0.0);
        assert_eq!(normal[2], -1.0);
    }
}
