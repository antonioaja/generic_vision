pub mod control;
pub mod misc;
mod tests;

use anyhow::*;
use image::EncodableLayout;
use rgb::FromSlice;

use crate::control::colorspaces::HSV;
use crate::misc::helpers::*;

use rayon::prelude::*;
use std::time::Instant;

fn main() -> Result<()> {
    let now = Instant::now();

    let ope = image::open("many_bodes.png").context("Could not open test_rotate.png")?;

    let color_test = control::tools::ColorArea::new(
        Dimensions {
            width: 34,
            height: 50,
        },
        Point { x: 216, y: 44 },
        Range {
            lower: 90.0,
            upper: 270.0,
        },
        Range {
            lower: 0.10,
            upper: 1.0,
        },
        Range {
            lower: 0.30,
            upper: 1.0,
        },
        Identification {
            name: "bluey",
            id: 1,
        },
    );

    let ope_hsv = ope
        .clone()
        .into_rgb8()
        .as_bytes()
        .as_rgb()
        .par_iter()
        .map(|x| HSV::from_rgb8(*x))
        .collect();

    let color_match_percentage = color_test.check(ope_hsv, ope.width());

    println!("{}%", color_match_percentage * 100.0);

    println!("{} ms", now.elapsed().as_millis());

    Ok(())
}
