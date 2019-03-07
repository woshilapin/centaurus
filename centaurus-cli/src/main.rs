mod arguments;

use centaurus::SceneBuilder;
use image::{ImageBuffer, Rgb};
use crate::arguments::Arguments;
use structopt::StructOpt;

fn main() {
    let arguments = Arguments::from_args();

    let mut scene_builder = SceneBuilder::new();
    scene_builder.with_dimension(arguments.dimension);
    scene_builder.with_width(arguments.width);
    scene_builder.with_height(arguments.height);

    let scene = scene_builder.build();
    let image = scene.render();
    let image_buffer = ImageBuffer::from_fn(image.width() as u32, image.height() as u32, |x, y| {
        let color = image.color(x as usize, y as usize);
        Rgb([color.get_red(), color.get_green(), color.get_blue()])
    });

    if image::ImageRgb8(image_buffer).save(&arguments.output_filename).is_ok() {
        println!("{:?} saved!", &arguments.output_filename);
    }
}
