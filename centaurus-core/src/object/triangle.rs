use nalgebra::{Matrix, Matrix4, MatrixArray, U4, Vector3, Vector4};
use object::Intersect;
use object::Intersection;
use object::Ray;
use std::f64::EPSILON;
use std::option::Option;

pub struct Triangle {
    vertices: [Vector3<f64>; 3],
}

impl Triangle {
    pub fn new(vertices: [Vector3<f64>; 3]) -> Triangle {
        Triangle {
            vertices
        }
    }
}

impl Intersect for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let origin = Vector4::new(ray.origin[0], ray.origin[1], ray.origin[2], 1.0);
        let direction = Vector4::new(ray.direction[0], ray.direction[1], ray.direction[2], 0.0);
        let vertex_0 = Vector3::new(self.vertices[0][0], self.vertices[0][1], self.vertices[0][2]);
        let vertex_1 = Vector3::new(self.vertices[1][0], self.vertices[1][1], self.vertices[1][2]);
        let vertex_2 = Vector3::new(self.vertices[2][0], self.vertices[2][1], self.vertices[2][2]);
        let edge_1 = vertex_1 - vertex_0;
        let edge_2 = vertex_2 - vertex_0;
        let normal = edge_1.cross(&edge_2);
        let affine_matrix = Matrix4::new(
            edge_1[0], edge_2[0], normal[0], vertex_0[0],
            edge_1[1], edge_2[1], normal[1], vertex_0[1],
            edge_1[2], edge_2[2], normal[2], vertex_0[2],
            0.0, 0.0, 0.0, 1.0);
        let inverse_affine_matrix = match affine_matrix.try_inverse() {
            Some(i) => i,
            None => return None,
        };
        let new_origin = inverse_affine_matrix * origin;
        let new_direction = inverse_affine_matrix * direction;
        let t = -new_origin[2] / new_direction[2];
        let u = new_origin[0] + t * new_direction[0];
        let v = new_origin[1] + t * new_direction[1];
        let intersection = Intersection {
            position: Vector3::new(0.0, 0.0, 0.0),
        };
        if t >= 0.0 && u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
            Some(intersection)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use spectral::assert_that;
    use spectral::numeric::FloatAssertions;
    use super::*;


    #[test]
    fn no_intersection_with_perpendicular_plan() {
        let ray = Ray {
            origin: Vector3::new(0.0, 0.0, -1.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let triangle = Triangle {
            vertices: [
                Vector3::new(-1.0, 1.0, 0.0),
                Vector3::new(1.0, 1.0, 0.0),
                Vector3::new(0.0, 1.0, 1.0),
            ],
        };
        assert_eq!(triangle.intersect(&ray).is_none(), true);
    }

    #[test]
    fn intersection_outside_triangle() {
        let ray = Ray {
            origin: Vector3::new(2.0, 2.0, -1.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let triangle = Triangle {
            vertices: [
                Vector3::new(-1.0, 1.0, 0.0),
                Vector3::new(1.0, 1.0, 0.0),
                Vector3::new(1.0, -1.0, 0.0),
            ],
        };
        assert_eq!(triangle.intersect(&ray).is_none(), true);
    }

    #[test]
    fn intersection_inside_symetric_triangle() {
        let ray = Ray {
            origin: Vector3::new(-1.0, -1.0, -1.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let triangle = Triangle {
            vertices: [
                Vector3::new(-1.0, 1.0, 0.0),
                Vector3::new(1.0, 1.0, 0.0),
                Vector3::new(1.0, -1.0, 0.0),
            ],
        };
        assert_eq!(triangle.intersect(&ray).is_none(), true);
    }

    #[test]
    fn intersection_with_triangle() {
        let ray = Ray {
            origin: Vector3::new(0.0, 0.0, -1.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let triangle = Triangle {
            vertices: [
                Vector3::new(-1.0, 0.0, 0.0),
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
            ],
        };
        assert_eq!(triangle.intersect(&ray).is_some(), true);
    }
}