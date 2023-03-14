pub mod control;
pub mod misc;
pub mod settings;

use anyhow::*;
use image::EncodableLayout;
use rgb::FromSlice;

use altered_perception::Luma;
use crate::misc::helpers::*;

use rayon::prelude::*;
use std::io::Write;
use std::{fs::File, time::Instant};

fn main() -> Result<()> {
    let ope = image::open("edge.png").context("Could not open test_rotate.png")?;

    let mut out = File::create("settings.txt").context("Could not create settings.txt")?;

    write!(
        out,
        "{:?}",
        settings::Settings::get_model_info("PROGRAM_1903.toml".to_string())
    )
    .context("Could not write to settings.txt")?;

    // println!(
    //     "{:?}",
    //     settings::Settings::get_model_info("PROGRAM_1903.toml".to_string())
    // );

    let now = Instant::now();

    let _color_test = control::tools::ColorArea::new(
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
            name: "bluey".to_string(),
            id: 1,
        },
    );

    let _some_width = ope.width();
    let _some_height = ope.height();

    // let lol = PositionAdjust::find_curl(
    //     ope.into_rgb8().as_bytes().as_rgb().to_vec(),
    //     Dimensions {
    //         width: some_width,
    //         height: some_height,
    //     },
    // );

    // let ope_hsv: Vec<HSV<f64>> = ope
    //     .into_rgb8()
    //     .as_bytes()
    //     .as_rgb()
    //     .par_iter()
    //     .map(|x| HSV::from_rgb(*x))
    //     .collect();
    let _ope_luma: Vec<Luma<u8>> = ope
        .into_rgb8()
        .as_bytes()
        .as_rgb()
        .par_iter()
        .map(|x| Luma::from_rgb(*x))
        .collect();

    //println!("{:?}", ope_luma);

    // println!("{} ms", now.elapsed().as_millis());

    // let color_match_percentage = color_test.check(ope_hsv, some_width);

    // println!("{}%", color_match_percentage * 100.0);

    println!("{} ms", now.elapsed().as_millis());

    Ok(())
}
