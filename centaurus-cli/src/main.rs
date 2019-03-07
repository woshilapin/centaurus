use centaurus::SceneBuilder;
use clap_verbosity_flag::Verbosity;
use image::{ImageBuffer, Rgb};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Centaurus",
    version = "0.1.0",
    author = "woshilapin <woshilapin@tuziwo.info",
    about = "A relativist ray-tracer"
)]
struct CentaurusArguments {
    #[structopt(
        name = "dimension",
        short = "d",
        long = "dimension",
        default_value = "3",
        help = "Spatial dimension of the scene",
        takes_value = true
    )]
    dimension: u8,
    #[structopt(
        name = "width",
        short = "w",
        long = "width",
        default_value = "600",
        help = "Width of the final output images",
        takes_value = true
    )]
    width: usize,
    #[structopt(
        name = "height",
        short = "h",
        long = "height",
        default_value = "600",
        help = "Height of the final output images",
        takes_value = true
    )]
    height: usize,
    #[structopt(
        name = "output-file",
        short = "o",
        long = "output-file",
        default_value = "centaurus.png",
        help = "Path for the output file",
        parse(from_os_str),
        takes_value = true
    )]
    output_filename: PathBuf,
    #[structopt(flatten)]
    verbose: Verbosity,
}

fn main() {
    let arguments = CentaurusArguments::from_args();

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
