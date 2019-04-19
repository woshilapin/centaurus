mod arguments;
mod properties;

use crate::arguments::Arguments;
use crate::properties::Properties;
use image::{ImageBuffer, Rgb};
use std::fs::File;
use structopt::StructOpt;

fn main() {
    let arguments = Arguments::from_args();

    let scene_file = File::open(arguments.input_filename).expect("Couldn't read the input file.");
    let properties_from_file: Properties =
        serde_yaml::from_reader(scene_file).expect("Couldn't parse the YAML file.");

    let scene = properties_from_file.scene;
    let image = scene.render();
    let image_buffer = ImageBuffer::from_fn(image.width() as u32, image.height() as u32, |x, y| {
        let color = image.color(x as usize, y as usize);
        Rgb([color.get_red(), color.get_green(), color.get_blue()])
    });

    if image::ImageRgb8(image_buffer)
        .save(&arguments.output_filename)
        .is_ok()
    {
        println!("{:?} saved!", &arguments.output_filename);
    }
}
