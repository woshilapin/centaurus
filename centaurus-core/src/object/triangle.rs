extern crate spectral;

use object::Intersect;
use object::Intersection;
use object::Ray;
use std::f64::EPSILON;
use std::option::Option;

pub struct Triangle {
    vertices: [[f64; 3]; 3],
}

impl Triangle {
    pub fn new(vertices: [[f64; 3]; 3]) -> Triangle {
        Triangle {
            vertices
        }
    }
}

fn vector(p1: &[f64; 3], p2: &[f64; 3]) -> [f64; 3] {
    [
        p2[0] - p1[0],
        p2[1] - p1[1],
        p2[2] - p1[2],
    ]
}

fn dot_product(v1: &[f64; 3], v2: &[f64; 3]) -> f64 {
    v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
}

fn cross_product(v1: &[f64; 3], v2: &[f64; 3]) -> [f64; 3] {
    [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ]
}

fn sub_matrix_2(matrix: &[[f64; 2]; 2], row: usize, col: usize) -> f64 {
    let mut sub = 0.0;
    let mut r = 0;
    let mut c = 0;
    for i in 0..2 {
        if i == row {
            continue;
        }
        for j in 0..2 {
            if j == col {
                continue;
            }
            sub = matrix[i][j];
            c = c + 1;
        }
        c = 0;
        r = r + 1;
    }
    sub
}

fn sub_matrix_3(matrix: &[[f64; 3]; 3], row: usize, col: usize) -> [[f64; 2]; 2] {
    let mut sub = [[0.0; 2], [0.0; 2]];
    let mut r = 0;
    let mut c = 0;
    for i in 0..3 {
        if i == row {
            continue;
        }
        for j in 0..3 {
            if j == col {
                continue;
            }
            sub[r][c] = matrix[i][j];
            c = c + 1;
        }
        c = 0;
        r = r + 1;
    }
    sub
}

fn sub_matrix_4(matrix: &[[f64; 4]; 4], row: usize, col: usize) -> [[f64; 3]; 3] {
    let mut sub = [[0.0; 3], [0.0; 3], [0.0; 3]];
    let mut r = 0;
    let mut c = 0;
    for i in 0..4 {
        if i == row {
            continue;
        }
        for j in 0..4 {
            if j == col {
                continue;
            }
            sub[r][c] = matrix[i][j];
            c = c + 1;
        }
        c = 0;
        r = r + 1;
    }
    sub
}

fn determinant_2(matrix: &[[f64; 2]; 2]) -> f64 {
    let mut det = 0.0;
    for n in 0..2 as usize {
        let sign = f64::powi(-1.0, n as i32);
        let sub_det = sub_matrix_2(&matrix, 0, n);
        det = det + sign * matrix[0][n] * sub_det;
    }
    det
}

fn determinant_3(matrix: &[[f64; 3]; 3]) -> f64 {
    let mut det = 0.0;
    for n in 0..3 as usize {
        let sign = f64::powi(-1.0, n as i32);
        let sub_det = sub_matrix_3(&matrix, 0, n);
        det = det + sign * matrix[0][n] * determinant_2(&sub_det);
    }
    det
}

fn determinant_4(matrix: &[[f64; 4]; 4]) -> f64 {
    let mut det = 0.0;
    for n in 0..4 as usize {
        let sign = f64::powi(-1.0, n as i32);
        let sub_det = sub_matrix_4(&matrix, 0, n);
        det = det + sign * matrix[0][n] * determinant_3(&sub_det);
    }
    det
}

fn co_factor_matrix(matrix: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut co_factor = [
        [0.0; 4],
        [0.0; 4],
        [0.0; 4],
        [0.0; 4],
    ];
    for i in 0..4 as usize {
        for j in 0..4 as usize {
            let sign = f64::powi(-1.0, (i + j) as i32);
            co_factor[i][j] = sign * determinant_3(&sub_matrix_4(&matrix, i, j));
        }
    }
    co_factor
}

fn transpose_matrix(matrix: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut transpose = [
        [0.0; 4],
        [0.0; 4],
        [0.0; 4],
        [0.0; 4],
    ];
    for i in 0..4 as usize {
        for j in 0..4 as usize {
            transpose[j][i] = matrix[i][j];
        }
    }
    transpose
}

fn adjoint_matrix(matrix: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    transpose_matrix(&co_factor_matrix(&matrix))
}

fn matrix_mul_scalar(matrix: &[[f64; 4]; 4], scalar: f64) -> [[f64; 4]; 4] {
    let mut product = [
        [0.0; 4],
        [0.0; 4],
        [0.0; 4],
        [0.0; 4],
    ];
    for i in 0..4 as usize {
        for j in 0..4 as usize {
            product[i][j] = matrix[i][j] * scalar;
        }
    }
    product
}

fn matrix_mul_matrix(matrix1: &[[f64; 4]; 4], matrix2: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut product = [
        [0.0; 4],
        [0.0; 4],
        [0.0; 4],
        [0.0; 4],
    ];
    for i in 0..4 as usize {
        for j in 0..4 as usize {
            for k in 0..4 as usize {
                product[i][j] += matrix1[i][k] * matrix2[k][j];
            }
        }
    }
    product
}

fn inverse_matrix(matrix: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    matrix_mul_scalar(&adjoint_matrix(&matrix), 1.0 / determinant_4(&matrix))
}

fn matrix_mul_vector(matrix: &[[f64; 4]; 4], vector: &[f64; 4]) -> [f64; 4] {
    let mut product = [0.0; 4];
    for i in 0..4 as usize {
        for k in 0..4 as usize {
            product[i] += matrix[i][k] * vector[k];
        }
    }
    product
}

impl Intersect for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let vertex_0 = self.vertices[0];
        let vertex_1 = self.vertices[1];
        let vertex_2 = self.vertices[2];
        let edge_1 = vector(&vertex_0, &vertex_1);
        let edge_2 = vector(&vertex_0, &vertex_2);
        let normal = cross_product(&edge_1, &edge_2);
        let affine_matrix = [
            [edge_1[0], edge_2[0], normal[0], vertex_0[0]],
            [edge_1[1], edge_2[1], normal[1], vertex_0[1]],
            [edge_1[2], edge_2[2], normal[2], vertex_0[2]],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let inverse_affine_matrix = inverse_matrix(&affine_matrix);
        let origin = [ray.origin()[0], ray.origin()[1], ray.origin()[2], 1.0];
        let direction = [ray.direction()[0], ray.direction()[1], ray.direction()[2], 0.0];
        let new_origin = matrix_mul_vector(&inverse_affine_matrix, &origin);
        let new_direction = matrix_mul_vector(&inverse_affine_matrix, &direction);
        let t = -new_origin[2] / new_direction[2];
        let u = new_origin[0] + t * new_direction[0];
        let v = new_origin[1] + t * new_direction[1];
        let intersection = Intersection {
            position: [0.0, 0.0, 0.0],
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
    use super::*;
    use super::spectral::assert_that;
    use super::spectral::numeric::FloatAssertions;

    #[test]
    fn dot_product_zero() {
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];
        assert_eq!(dot_product(&v1, &v2), 0.0);
    }

    #[test]
    fn dot_product_one() {
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [1.0, 0.0, 0.0];
        assert_eq!(dot_product(&v1, &v2), 1.0);
    }

    #[test]
    fn dot_product_random() {
        let v1 = [1.0, 3.0, -5.0];
        let v2 = [4.0, -2.0, -1.0];
        assert_eq!(dot_product(&v1, &v2), 3.0);
    }

    #[test]
    fn cross_product_orthogonal() {
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];
        let cross = cross_product(&v1, &v2);
        assert_eq!(cross[0], 0.0);
        assert_eq!(cross[1], 0.0);
        assert_eq!(cross[2], 1.0);
    }

    #[test]
    fn cross_product_random() {
        let v1 = [2.0, 3.0, 4.0];
        let v2 = [5.0, 6.0, 7.0];
        let cross = cross_product(&v1, &v2);
        assert_eq!(cross[0], -3.0);
        assert_eq!(cross[1], 6.0);
        assert_eq!(cross[2], -3.0);
    }

    #[test]
    fn sub_matrix_2_2() {
        let matrix = [
            [1.0, 2.0],
            [4.0, 8.0],
        ];
        let sub = sub_matrix_2(&matrix, 1, 0);
        assert_eq!(sub, 2.0);
    }

    #[test]
    fn sub_matrix_3_3() {
        let matrix = [
            [1.0, 3.0, 5.0],
            [2.0, 4.0, 6.0],
            [3.0, 5.0, 7.0],
        ];
        let sub = sub_matrix_3(&matrix, 2, 0);
        assert_eq!(sub[0][0], 3.0);
        assert_eq!(sub[0][1], 5.0);
        assert_eq!(sub[1][0], 4.0);
        assert_eq!(sub[1][1], 6.0);
    }

    #[test]
    fn sub_matrix_4_4() {
        let matrix = [
            [1.0, 2.0, 3.0, 4.0],
            [3.0, 4.0, 5.0, 6.0],
            [5.0, 6.0, 7.0, 8.0],
            [7.0, 8.0, 9.0, 10.0],
        ];
        let sub_matrix = sub_matrix_4(&matrix, 1, 2);
        assert_eq!(sub_matrix[0][0], 1.0);
        assert_eq!(sub_matrix[0][1], 2.0);
        assert_eq!(sub_matrix[0][2], 4.0);
        assert_eq!(sub_matrix[1][0], 5.0);
        assert_eq!(sub_matrix[1][1], 6.0);
        assert_eq!(sub_matrix[1][2], 8.0);
        assert_eq!(sub_matrix[2][0], 7.0);
        assert_eq!(sub_matrix[2][1], 8.0);
        assert_eq!(sub_matrix[2][2], 10.0);
    }

    #[test]
    fn determinant_2_random() {
        let matrix = [
            [3.0, 8.0],
            [4.0, 6.0],
        ];
        let det = determinant_2(&matrix);
        assert_eq!(det, -14.0);
    }

    #[test]
    fn determinant_3_random() {
        let matrix = [
            [-2.0, 2.0, -3.0],
            [-1.0, 1.0, 3.0],
            [2.0, 0.0, -1.0],
        ];
        let det = determinant_3(&matrix);
        assert_eq!(det, 18.0);
    }

    #[test]
    fn determinant_4_random() {
        let matrix = [
            [1.0, 2.0, 2.0, 1.0],
            [2.0, 7.0, 5.0, 2.0],
            [1.0, 2.0, 4.0, 2.0],
            [-1.0, 4.0, -6.0, 3.0],
        ];
        let det = determinant_4(&matrix);
        assert_eq!(det, 42.0);
    }

    #[test]
    fn co_factor_matrix_random() {
        let matrix = [
            [1.0, 2.0, 2.0, 1.0],
            [2.0, 7.0, 5.0, 2.0],
            [1.0, 2.0, 4.0, 2.0],
            [-1.0, 4.0, -6.0, 3.0],
        ];
        let co_factor = co_factor_matrix(&matrix);
        assert_eq!(co_factor[0][0], 122.0);
        assert_eq!(co_factor[0][1], -19.0);
        assert_eq!(co_factor[0][2], -27.0);
        assert_eq!(co_factor[0][3], 12.0);
        assert_eq!(co_factor[1][0], -24.0);
        assert_eq!(co_factor[1][1], 12.0);
        assert_eq!(co_factor[1][2], 6.0);
        assert_eq!(co_factor[1][3], -12.0);
        assert_eq!(co_factor[2][0], -34.0);
        assert_eq!(co_factor[2][1], -4.0);
        assert_eq!(co_factor[2][2], 12.0);
        assert_eq!(co_factor[2][3], 18.0);
        assert_eq!(co_factor[3][0], -2.0);
        assert_eq!(co_factor[3][1], 1.0);
        assert_eq!(co_factor[3][2], -3.0);
        assert_eq!(co_factor[3][3], 6.0);
    }

    #[test]
    fn transpose_matrix_random() {
        let matrix = [
            [1.0, 2.0, 2.0, 1.0],
            [2.0, 7.0, 5.0, 2.0],
            [1.0, 2.0, 4.0, 2.0],
            [-1.0, 4.0, -6.0, 3.0],
        ];
        let transpose = transpose_matrix(&matrix);
        assert_eq!(transpose[0][0], 1.0);
        assert_eq!(transpose[0][1], 2.0);
        assert_eq!(transpose[0][2], 1.0);
        assert_eq!(transpose[0][3], -1.0);
        assert_eq!(transpose[1][0], 2.0);
        assert_eq!(transpose[1][1], 7.0);
        assert_eq!(transpose[1][2], 2.0);
        assert_eq!(transpose[1][3], 4.0);
        assert_eq!(transpose[2][0], 2.0);
        assert_eq!(transpose[2][1], 5.0);
        assert_eq!(transpose[2][2], 4.0);
        assert_eq!(transpose[2][3], -6.0);
        assert_eq!(transpose[3][0], 1.0);
        assert_eq!(transpose[3][1], 2.0);
        assert_eq!(transpose[3][2], 2.0);
        assert_eq!(transpose[3][3], 3.0);
    }

    #[test]
    fn adjoint_matrix_random() {
        let matrix = [
            [1.0, 2.0, 2.0, 1.0],
            [2.0, 7.0, 5.0, 2.0],
            [1.0, 2.0, 4.0, 2.0],
            [-1.0, 4.0, -6.0, 3.0],
        ];
        let adjoint = adjoint_matrix(&matrix);
        assert_eq!(adjoint[0][0], 122.0);
        assert_eq!(adjoint[0][1], -24.0);
        assert_eq!(adjoint[0][2], -34.0);
        assert_eq!(adjoint[0][3], -2.0);
        assert_eq!(adjoint[1][0], -19.0);
        assert_eq!(adjoint[1][1], 12.0);
        assert_eq!(adjoint[1][2], -4.0);
        assert_eq!(adjoint[1][3], 1.0);
        assert_eq!(adjoint[2][0], -27.0);
        assert_eq!(adjoint[2][1], 6.0);
        assert_eq!(adjoint[2][2], 12.0);
        assert_eq!(adjoint[2][3], -3.0);
        assert_eq!(adjoint[3][0], 12.0);
        assert_eq!(adjoint[3][1], -12.0);
        assert_eq!(adjoint[3][2], 18.0);
        assert_eq!(adjoint[3][3], 6.0);
    }

    #[test]
    fn matrix_mul_scalar_random() {
        let matrix = [
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 5.0, 8.0, 11.0],
            [3.0, 7.0, 11.0, 15.0],
            [-1.0, -2.0, -4.0, -8.0],
        ];
        let product = matrix_mul_scalar(&matrix, 0.5);
        assert_eq!(product[0][0], 0.5);
        assert_eq!(product[0][1], 1.0);
        assert_eq!(product[0][2], 2.0);
        assert_eq!(product[0][3], 4.0);
        assert_eq!(product[1][0], 1.0);
        assert_eq!(product[1][1], 2.5);
        assert_eq!(product[1][2], 4.0);
        assert_eq!(product[1][3], 5.5);
        assert_eq!(product[2][0], 1.5);
        assert_eq!(product[2][1], 3.5);
        assert_eq!(product[2][2], 5.5);
        assert_eq!(product[2][3], 7.5);
        assert_eq!(product[3][0], -0.5);
        assert_eq!(product[3][1], -1.0);
        assert_eq!(product[3][2], -2.0);
        assert_eq!(product[3][3], -4.0);
    }

    #[test]
    fn matrix_mul_matrix_random() {
        let matrix1 = [
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 5.0, 8.0, 11.0],
            [3.0, 7.0, 11.0, 15.0],
            [-1.0, -2.0, -4.0, -8.0],
        ];
        let matrix2 = [
            [-1.0, 2.0, 1.0, -2.0],
            [1.0, 1.0, 1.0, 1.0],
            [-1.0, 1.0, -1.0, 1.0],
            [-2.0, 1.0, -2.0, 1.0],
        ];
        let product = matrix_mul_matrix(&matrix1, &matrix2);
        assert_eq!(product[0][0], -19.0);
        assert_eq!(product[0][1], 16.0);
        assert_eq!(product[0][2], -17.0);
        assert_eq!(product[0][3], 12.0);
        assert_eq!(product[1][0], -27.0);
        assert_eq!(product[1][1], 28.0);
        assert_eq!(product[1][2], -23.0);
        assert_eq!(product[1][3], 20.0);
        assert_eq!(product[2][0], -37.0);
        assert_eq!(product[2][1], 39.0);
        assert_eq!(product[2][2], -31.0);
        assert_eq!(product[2][3], 27.0);
        assert_eq!(product[3][0], 19.0);
        assert_eq!(product[3][1], -16.0);
        assert_eq!(product[3][2], 17.0);
        assert_eq!(product[3][3], -12.0);
    }

    #[test]
    fn matrix_inverse_random() {
        let matrix = [
            [1.0, 2.0, 2.0, 1.0],
            [2.0, 7.0, 5.0, 2.0],
            [1.0, 2.0, 4.0, 2.0],
            [-1.0, 4.0, -6.0, 3.0],
        ];
        let identity = matrix_mul_matrix(&matrix, &inverse_matrix(&matrix));
        assert_that(&identity[0][0]).is_close_to(1.0, 10.0 * EPSILON);
        assert_that(&identity[0][1]).is_close_to(0.0, 10.0 * EPSILON);
        assert_that(&identity[0][2]).is_close_to(0.0, 10.0 * EPSILON);
        assert_that(&identity[0][3]).is_close_to(0.0, 10.0 * EPSILON);
        assert_that(&identity[1][0]).is_close_to(0.0, 10.0 * EPSILON);
        assert_that(&identity[1][1]).is_close_to(1.0, 10.0 * EPSILON);
        assert_that(&identity[1][2]).is_close_to(0.0, 10.0 * EPSILON);
        assert_that(&identity[1][3]).is_close_to(0.0, 10.0 * EPSILON);
        assert_that(&identity[2][0]).is_close_to(0.0, 10.0 * EPSILON);
        assert_that(&identity[2][1]).is_close_to(0.0, 10.0 * EPSILON);
        assert_that(&identity[2][2]).is_close_to(1.0, 10.0 * EPSILON);
        assert_that(&identity[2][3]).is_close_to(0.0, 10.0 * EPSILON);
        assert_that(&identity[3][0]).is_close_to(0.0, 10.0 * EPSILON);
        assert_that(&identity[3][1]).is_close_to(0.0, 10.0 * EPSILON);
        assert_that(&identity[3][2]).is_close_to(0.0, 10.0 * EPSILON);
        assert_that(&identity[3][3]).is_close_to(1.0, 10.0 * EPSILON);
    }

    #[test]
    fn matrix_mul_vector_random() {
        let matrix = [
            [1.0, 2.0, 3.0, 4.0],
            [3.0, 7.0, 5.0, 1.0],
            [2.0, 2.0, 9.0, 4.0],
            [1.0, 2.0, 1.0, 2.0],
        ];
        let vector = [1.0, 2.0, 1.0, 1.0];
        let result = matrix_mul_vector(&matrix, &vector);
        assert_eq!(result[0], 12.0);
        assert_eq!(result[1], 23.0);
        assert_eq!(result[2], 19.0);
        assert_eq!(result[3], 8.0);
    }

    #[test]
    fn no_intersection_with_perpendicular_plan() {
        let ray = Ray {
            origin: [0.0, 0.0, -1.0],
            direction: [0.0, 0.0, 1.0],
        };
        let triangle = Triangle {
            vertices: [
                [-1.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
                [0.0, 1.0, 1.0],
            ],
        };
        assert_eq!(triangle.intersect(&ray).is_none(), true);
    }

    #[test]
    fn intersection_outside_triangle() {
        let ray = Ray {
            origin: [2.0, 2.0, -1.0],
            direction: [0.0, 0.0, 1.0],
        };
        let triangle = Triangle {
            vertices: [
                [-1.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
                [1.0, -1.0, 0.0],
            ],
        };
        assert_eq!(triangle.intersect(&ray).is_none(), true);
    }

    #[test]
    fn intersection_inside_symetric_triangle() {
        let ray = Ray {
            origin: [-1.0, -1.0, -1.0],
            direction: [0.0, 0.0, 1.0],
        };
        let triangle = Triangle {
            vertices: [
                [-1.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
                [1.0, -1.0, 0.0],
            ],
        };
        assert_eq!(triangle.intersect(&ray).is_none(), true);
    }

    #[test]
    fn intersection_with_triangle() {
        let ray = Ray {
            origin: [0.0, 0.0, -1.0],
            direction: [0.0, 0.0, 1.0],
        };
        let triangle = Triangle {
            vertices: [
                [-1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
            ],
        };
        assert_eq!(triangle.intersect(&ray).is_some(), true);
    }
}