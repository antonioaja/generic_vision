pub mod control;
pub mod misc;
mod tests;

use anyhow::*;
use control::tools::Point;
use image::EncodableLayout;
use rgb::FromSlice;

use crate::control::tools::Dimensions;
// use clap::Parser;
// use imageproc::geometric_transformations::rotate_about_center;
// use imageproc::geometric_transformations::Interpolation;

// use crate::misc::helpers::get_extension;
// use crate::misc::helpers::Args;

fn main() -> Result<()> {
    let ope = image::open("4k_rain.png").context("Could not open test_rotate.png")?;

    let color_test = control::tools::ColorArea::new(
        Dimensions {
            width: ope.width(),
            height: ope.height(),
        },
        Point { x: 0, y: 0 },
        [0.0, 180.0],
        [0.0, 1.0],
        [0.0, 1.0],
        1,
        "4k_rain",
    );

    let color_match_percentage =
        color_test.check(ope.clone().into_rgb8().as_bytes().as_rgb(), ope.width());

    println!("{}", color_match_percentage);

    Ok(())
}
