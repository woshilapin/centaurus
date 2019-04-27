use crate::ray::Ray;
use crate::render::Renderer;
use crate::scene::Scene;
use image::{ImageBuffer, Rgba};
use indicatif::ProgressBar;
use nalgebra::{Point3, Vector3};

fn bounded_add(v1: f64, v2: f64) -> f64 {
    if v1 + v2 <= 1.0f64 {
        v1 + v2
    } else {
        1.0f64
    }
}
fn bounded_multiply(value: f64, scalar: f64) -> f64 {
    value * scalar
}
fn multiply_color_scalar(c: Rgba<f64>, scalar: f64) -> Rgba<f64> {
    Rgba([
        bounded_multiply(c[0], scalar),
        bounded_multiply(c[1], scalar),
        bounded_multiply(c[2], scalar),
        c[3],
    ])
}
fn combine_color(c1: Rgba<f64>, c2: Rgba<f64>) -> Rgba<f64> {
    Rgba([
        bounded_add(c1[0], c2[0]),
        bounded_add(c1[1], c2[1]),
        bounded_add(c1[2], c2[2]),
        bounded_add(c1[3], c2[3]),
    ])
}

pub struct DefaultRenderer;

impl Renderer for DefaultRenderer {
    fn render(scene: &Scene) -> ImageBuffer<Rgba<f64>, Vec<f64>> {
        let progress_bar = ProgressBar::new(u64::from(scene.height) * u64::from(scene.width));
        ImageBuffer::from_fn(scene.width, scene.height, |i, j| {
            progress_bar.inc(1);
            let x_unit =
                (scene.camera.right_bound - scene.camera.left_bound) / f64::from(scene.width);
            let y_unit =
                (scene.camera.upper_bound - scene.camera.lower_bound) / f64::from(scene.height);
            let x = scene.camera.left_bound + (f64::from(i) + 0.5f64) * x_unit;
            let y = scene.camera.lower_bound + (f64::from(scene.height - j) - 0.5f64) * y_unit;
            let ray = Ray::new(Point3::new(x, y, -1.0), Vector3::new(0.0, 0.0, 1.0));
            let mut color = Rgba([
                scene.background_color[0],
                scene.background_color[1],
                scene.background_color[2],
                1.0f64,
            ]);
            for object in &scene.objects {
                if let Some(intersection) = object.intersect(&ray) {
                    let i_position = intersection.position;
                    let i_normal = intersection.normal;
                    for light in &scene.lights {
                        if let Some((l_direction, l_color)) = light.hit(&i_position) {
                            let intensity = i_normal.dot(&(-l_direction));
                            if intensity >= 0.0 && intensity <= 1.0 {
                                let l_color = Rgba([l_color[0], l_color[1], l_color[2], 1.0f64]);
                                let new_color = multiply_color_scalar(l_color, intensity);
                                color = combine_color(color, new_color);
                            }
                        }
                    }
                }
            }
            color
        })
    }
}
