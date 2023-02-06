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
    let ope = image::open("OAKD-0014.png").context("Could not open test_rotate.png")?;

    let now = Instant::now();

    let color_test = control::tools::ColorArea::new(
        Dimensions {
            width: 60,
            height: 140,
        },
        Point { x: 1060, y: 420 },
        Range {
            lower: 175.0,
            upper: 265.0,
        },
        Range {
            lower: 0.50,
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

    let some_width = ope.width();

    let ope_hsv = ope
        .into_rgb8()
        .as_bytes()
        .as_rgb()
        .par_iter()
        .map(|x| HSV::from_rgb(*x))
        .collect();

    println!("{} ms", now.elapsed().as_millis());

    let color_match_percentage = color_test.check(ope_hsv, some_width);

    println!("{}%", color_match_percentage * 100.0);

    println!("{} ms", now.elapsed().as_millis());

    Ok(())
}
