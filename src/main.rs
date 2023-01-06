pub mod helpers;
pub mod numerical_methods;

use anyhow::*;
use clap::Parser;
use imageproc::geometric_transformations::rotate_about_center;
use imageproc::geometric_transformations::Interpolation;

use crate::helpers::get_extension;
use crate::helpers::Args;

fn main() -> Result<()> {
    // Collect arguments
    let args = Args::parse();
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
        image::Luma([0]),
    );

    rotated
        .save("rotate.png")
        .context("Could not save rotated image")?;

    let attr = dssim::Dssim::new();
    let im1 = dssim::load_image(&attr, &output_file)
        .context(format!("Could not load {}", output_file))?;
    let im2 = dssim::load_image(&attr, "rotate.png").context("Could not open rotate.png")?;

    let (diff, _) = attr.compare(&im1, im2);

    println!("{}", diff);

    Ok(())
}
