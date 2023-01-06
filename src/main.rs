use anyhow::*;
use clap::Parser;
use imageproc::geometric_transformations::rotate_about_center;
use imageproc::geometric_transformations::Interpolation;
use std::ffi::OsStr;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(version, about = "A generic vision program.", long_about = None)]
struct Args {
    /// The .dxf file to convert
    #[clap(short, long, value_parser)]
    input_file: String,

    /// Activates verbose output, eliminates .elmt file writing
    #[clap(short, long, value_parser, default_value = "")]
    output_file: String,
}

fn main() -> Result<()> {
    // Collect arguments
    let args: Args = Args::parse();
    let input_file = &args.input_file;
    let mut output_file = args.output_file;

    if output_file.is_empty() {
        output_file = input_file.replace(&format!(".{}", get_extension(input_file)?), "_edge.png");
    }

    let src_image = image::open(input_file).context(format!("Could not open {}", input_file))?;

    let detection = edge_detection::canny(src_image.to_luma8(), 2.0, 0.2, 0.01);

    detection
        .as_image()
        .save(&output_file)
        .context(format!("Could not save {}", output_file))?;

    let rotated = rotate_about_center(
        &detection.as_image().to_luma8(),
        std::f32::consts::FRAC_PI_3,
        Interpolation::Nearest,
        image::Luma([1]),
    );

    rotated
        .save("rotate.png")
        .context("Could not save rotated image")?;

    Ok(())
}

fn get_extension(filename: &str) -> Result<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .context(format!("Could not find extension for {}", filename))
}
