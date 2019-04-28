use crate::object::Intersection;
use crate::Object;
use crate::{Ray, Vertex};
use nalgebra::{Matrix4, Vector3};
use serde_derive::Deserialize;
use std::option::Option;

#[derive(Debug, Deserialize)]
pub struct Triangle {
    vertices: [Vertex<f64>; 3],
}

impl Triangle {
    pub fn new(vertices: [Vertex<f64>; 3]) -> Triangle {
        Triangle { vertices }
    }
}

#[typetag::deserialize(name = "triangle")]
impl Object for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        trace!("Searching for intersection with {:?}", ray);
        let origin = ray.origin.to_homogeneous();
        let direction = ray.direction.to_homogeneous();
        let edge_1 = self.vertices[1].position - self.vertices[0].position;
        let edge_2 = self.vertices[2].position - self.vertices[0].position;
        let affine_matrix = Matrix4::from_columns(&[
            edge_1.to_homogeneous(),
            edge_2.to_homogeneous(),
            direction,
            self.vertices[0].position.to_homogeneous(),
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
            let normal_0 = (1.0 - u - v) * self.vertices[0].normal.to_homogeneous();
            let normal_1 = u * self.vertices[1].normal.to_homogeneous();
            let normal_2 = v * self.vertices[2].normal.to_homogeneous();
            let normal = normal_0 + normal_1 + normal_2;
            let normal = match Vector3::from_homogeneous((normal).normalize()) {
                Some(i) => i,
                None => return None,
            };
            let intersection = Intersection {
                position: ray.origin + t * ray.direction,
                normal,
            };
            trace!("Found {:?}", intersection);
            Some(intersection)
        } else {
            trace!("No intersection found");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point3;
    use pretty_assertions::assert_eq;

    #[test]
    fn no_intersection_with_perpendicular_plan() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
        let triangle = Triangle {
            vertices: [
                Vertex::new(Point3::new(-1.0, 1.0, 0.0), Vector3::new(0.0, 0.0, -1.0)),
                Vertex::new(Point3::new(1.0, 1.0, 0.0), Vector3::new(0.0, 0.0, -1.0)),
                Vertex::new(Point3::new(0.0, 1.0, 1.0), Vector3::new(0.0, 0.0, -1.0)),
            ],
        };
        let intersection = triangle.intersect(&ray);
        assert_eq!(intersection.is_none(), true);
    }

    #[test]
    fn intersection_outside_triangle() {
        let ray = Ray::new(Point3::new(2.0, 2.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
        let triangle = Triangle {
            vertices: [
                Vertex::new(Point3::new(-1.0, 1.0, 0.0), Vector3::new(0.0, 0.0, -1.0)),
                Vertex::new(Point3::new(1.0, 1.0, 0.0), Vector3::new(0.0, 0.0, -1.0)),
                Vertex::new(Point3::new(0.0, 1.0, 1.0), Vector3::new(0.0, 0.0, -1.0)),
            ],
        };
        let intersection = triangle.intersect(&ray);
        assert_eq!(intersection.is_none(), true);
    }

    #[test]
    fn intersection_inside_symetric_triangle() {
        let ray = Ray::new(Point3::new(-1.0, -1.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
        let triangle = Triangle {
            vertices: [
                Vertex::new(Point3::new(-1.0, 1.0, 0.0), Vector3::new(0.0, 0.0, -1.0)),
                Vertex::new(Point3::new(1.0, 1.0, 0.0), Vector3::new(0.0, 0.0, -1.0)),
                Vertex::new(Point3::new(0.0, 1.0, 1.0), Vector3::new(0.0, 0.0, -1.0)),
            ],
        };
        let intersection = triangle.intersect(&ray);
        assert_eq!(intersection.is_none(), true);
    }

    #[test]
    fn intersection_with_triangle() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
        let triangle = Triangle {
            vertices: [
                Vertex::new(
                    Point3::new(0.0, 1.0, 0.0),
                    Vector3::new(0.0, 0.0, -1.0).normalize(),
                ),
                Vertex::new(
                    Point3::new(-1.0, 0.0, 0.0),
                    Vector3::new(-1.0, 0.0, -1.0).normalize(),
                ),
                Vertex::new(
                    Point3::new(1.0, 0.0, 0.0),
                    Vector3::new(1.0, 0.0, -1.0).normalize(),
                ),
            ],
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
