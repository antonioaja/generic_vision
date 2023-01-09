pub mod misc;
pub mod control;

use anyhow::*;
use clap::Parser;
use imageproc::geometric_transformations::rotate_about_center;
use imageproc::geometric_transformations::Interpolation;

use crate::misc::helpers::get_extension;
use crate::misc::helpers::Args;

fn main() -> Result<()> {
    // Collect arguments
    let args = Args::parse();
    let input_file = &args.input_file;
    let mut output_file = args.output_file;

    if output_file.is_empty() {
        output_file = input_file.replace(&format!(".{}", get_extension(input_file)?), "_edge.png");
    }

    let src_image = image::open(input_file).context(format!("Could not open {}", input_file))?;

    // let detection = edge_detection::canny(src_image.to_luma8(), 2.0, 0.2, 0.01);

    // detection
    //     .as_image()
    //     .save(&output_file)
    //     .context(format!("Could not save {}", output_file))?;

    // let mut increment = 1;

    // let attr = dssim::Dssim::new();
    // let im1 =
    //     dssim::load_image(&attr, input_file).context(format!("Could not load {}", output_file))?;
    // let im2 = dssim::load_image(&attr, "pi_3_rotate.png").context("Could not open rotate.png")?;

    // let (diff, _) = attr.compare(&im1, im2);

    // let mut bi_meth = numerical_methods::BisectionMethod::new(
    //     [0.0, diff.into()],
    //     [PI, 0.1],
    // );

    // loop {
    //     let sample = image::open(input_file).unwrap();
    //     let rotated = rotate_about_center(
    //         &sample.to_luma8(),
    //         bi_meth.calc_midpoint() as f32,
    //         Interpolation::Nearest,
    //         image::Luma([0]),
    //     );

    //     rotated
    //         .save("rotate.png")
    //         .context("Could not save rotated image")?;

    //     let attr = dssim::Dssim::new();
    //     let im1 = dssim::load_image(&attr, "pi_3_rotate.png")
    //         .context(format!("Could not load {}", output_file))?;
    //     let im2 = dssim::load_image(&attr, "rotate.png").context("Could not open rotate.png")?;

    //     let (diff, _) = attr.compare(&im1, im2);

    //     let results = bi_meth.iterate(diff.into());

    //     println!("{:?}", results);

    //     if results[1].abs() < 0.1 {
    //         break;
    //     }

    //     if increment > 100 {
    //         break;
    //     }

    //     increment += 1;
    // }

    Ok(())
}
