pub mod control;
pub mod misc;
mod tests;

use anyhow::*;
use image::EncodableLayout;
use rgb::FromSlice;

use crate::misc::helpers::*;

fn main() -> Result<()> {
    let ope = image::open("test_rotate.png").context("Could not open test_rotate.png")?;

    let color_test = control::tools::ColorArea::new(
        Dimensions {
            width: 34,
            height: 50,
        },
        Point { x: 216, y: 44 },
        Range {
            lower: 175.0,
            upper: 240.0,
        },
        Range {
            lower: 0.10,
            upper: 1.0,
        },
        Range {
            lower: 0.70,
            upper: 1.0,
        },
        1,
        "4k_rain",
    );

    let color_match_percentage =
        color_test.check(ope.clone().into_rgb8().as_bytes().as_rgb(), ope.width());

    println!("{}%", color_match_percentage * 100.0);

    Ok(())
}
