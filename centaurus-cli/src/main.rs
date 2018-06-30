extern crate centaurus_core;
extern crate clap;

use centaurus_core::SceneBuilder;
use clap::{App, Arg};
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::str::FromStr;
use std::string::String;
use std::u8;

fn is_integer_between<T: FromStr + PartialOrd + Debug>(string: String, min: T, max: T) -> Result<(), String> {
    match string.parse::<T>() {
        Ok(ref v) if *v >= min && *v <= max => Ok(()),
        Ok(ref v) => Err(format!("'{:?}' is invalid; dimension should be strictly greater than {:?} and less than {:?}.", *v, min, max)),
        Err(_) => Err(format!("'{}' is not a integer.", string)),
    }
}

fn main() {
    let matches = App::new("Centaurus")
        .version("0.1.0")
        .author("woshilapin <woshilapin@tuziwo.info")
        .about("A relativist ray-tracer")
        .arg(Arg::with_name("dimension")
            .short("d")
            .long("dimension")
            .value_name("INTEGER")
            .validator(|value| is_integer_between(value, 1, u8::MAX))
            .default_value("3")
            .help("Spatial dimension of the scene")
            .takes_value(true))
        .get_matches();

    let mut scene_builder = SceneBuilder::new();

    if let Some(d) = matches.value_of("dimension") {
        if let Ok(i) = d.parse() {
            scene_builder.with_dimension(i);
        }
    }

    let scene = scene_builder.build();

    println!("The dimension is {}", scene.render());
}
