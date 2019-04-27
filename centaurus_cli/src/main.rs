#[macro_use]
extern crate log;

mod arguments;
mod properties;

use crate::arguments::Arguments;
use crate::properties::Properties;
use image::{ImageBuffer, Rgba, RgbaImage};
use std::fs::File;
use structopt::StructOpt;

fn convert(image: ImageBuffer<Rgba<f64>, Vec<f64>>) -> RgbaImage {
    RgbaImage::from_fn(image.width(), image.height(), |x, y| {
        let color = image.get_pixel(x, y);
        let r = (color[0] * 255.0) as u8;
        let g = (color[1] * 255.0) as u8;
        let b = (color[2] * 255.0) as u8;
        let a = (color[3] * 255.0) as u8;
        Rgba([r, g, b, a])
    })
}

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let arguments = Arguments::from_args();

    let scene_file = File::open(arguments.input_filename).expect("Couldn't read the input file.");
    let properties_from_file: Properties =
        serde_yaml::from_reader(scene_file).expect("Couldn't parse the YAML file.");

    let scene = properties_from_file.scene;
    let image_buffer = scene.render();
    let image_buffer: RgbaImage = convert(image_buffer);
    if image::ImageRgba8(image_buffer)
        .save(&arguments.output_filename)
        .is_ok()
    {
        info!("File {:?} created.", &arguments.output_filename);
    }
}
