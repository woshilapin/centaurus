use clap_verbosity_flag::Verbosity;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Centaurus",
    version = "0.1.0",
    author = "woshilapin <woshilapin@tuziwo.info",
    about = "A relativist ray-tracer"
)]
pub struct Arguments {
    #[structopt(
        name = "dimension",
        short = "d",
        long = "dimension",
        default_value = "3",
        help = "Spatial dimension of the scene"
    )]
    pub dimension: u8,
    #[structopt(
        name = "width",
        short = "w",
        long = "width",
        default_value = "600",
        help = "Width of the final output images"
    )]
    pub width: usize,
    #[structopt(
        name = "height",
        short = "h",
        long = "height",
        default_value = "600",
        help = "Height of the final output images"
    )]
    pub height: usize,
    #[structopt(
        name = "output-file",
        short = "o",
        long = "output-file",
        default_value = "centaurus.png",
        help = "Path for the output file",
        parse(from_os_str)
    )]
    pub output_filename: PathBuf,
    #[structopt(
        name = "input-file",
        short = "i",
        long = "input-file",
        default_value = "centaurus.yml",
        help = "Path for the input configuration file",
        parse(from_os_str)
    )]
    pub input_filename: PathBuf,
    #[structopt(flatten)]
    pub verbose: Verbosity,
}
