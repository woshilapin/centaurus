use crate::Renderer;
use crate::{Intersection, Light, Object, Ray, Scene};
use image::{ImageBuffer, Rgb, Rgba};
use indicatif::ProgressBar;
use nalgebra::{Point3, Vector3};

pub struct RayTracerRenderer;

fn calculate_intersections(ray: &Ray, objects: &[Box<dyn Object>]) -> Vec<Intersection> {
    objects
        .iter()
        .map(|object| object.intersect(ray))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
}
fn bound<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    value
}

fn calculate_illumination(
    eye: &Ray,
    intersection: &Intersection,
    lights: &[Box<dyn Light>],
) -> Option<Rgb<f64>> {
    let position = intersection.position;
    let normal = intersection.normal;
    let mut final_color = None;
    for light in lights {
        if let Some((direction, color)) = light.hit(&position) {
            let intensity = normal.dot(&(-direction));
            if intensity >= 0.0f64 && intensity <= 1.0f64 {
                final_color = match final_color {
                    None => Some(Rgb([
                        color[0] * intensity,
                        color[1] * intensity,
                        color[2] * intensity,
                    ])),
                    Some(c) => Some(Rgb([
                        bound(c[0] + intensity * color[0], 0.0f64, 1.0f64),
                        bound(c[1] + intensity * color[1], 0.0f64, 1.0f64),
                        bound(c[2] + intensity * color[2], 0.0f64, 1.0f64),
                    ])),
                };
            }
        }
    }
    if final_color.is_none() && -eye.direction.dot(&intersection.normal) >= 0.0f64 {
        final_color = Some(Rgb([0.0f64, 0.0f64, 0.0f64]));
    }
    final_color
}

impl Renderer for RayTracerRenderer {
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
                0.0f64,
            ]);
            let mut intersections = calculate_intersections(&ray, &scene.objects);
            intersections.sort_unstable_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
            for intersection in &intersections {
                if let Some(illumination) =
                    calculate_illumination(&ray, intersection, &scene.lights)
                {
                    color = Rgba([illumination[0], illumination[1], illumination[2], 1.0f64]);
                    break;
                }
            }
            color
        })
    }
}
