pub mod control;
pub mod misc;
pub mod tests;

use anyhow::*;
use control::colorspaces::HSV;
use rgb::RGB;
// use clap::Parser;
// use imageproc::geometric_transformations::rotate_about_center;
// use imageproc::geometric_transformations::Interpolation;

// use crate::misc::helpers::get_extension;
// use crate::misc::helpers::Args;

fn main() -> Result<()> {
    // let mut test_model =
    //     control::model::Model::new("ope".to_string(), "test.png".to_string(), 10, 10);

    // let ope = image::open("test.png").context("Could not open test_rotate.png")?;

    // let mut var: f64 = 0.0;

    // test_model
    //     .find_curl(ope.clone(), &mut var)
    //     .context("Error during curl calculation")?;

    // test_model.find_offset(ope, var)?;

    // println!("{}", var);

    // let color_test = control::tools::ColorArea::new();
    // color_test.check(ope);

    let test = RGB::new(0, 1, 6);
    let test2 = HSV::from_rgb8(test);
    let test3 = HSV::to_rgb8(test2);
    println!("{:?}", test);
    println!("{:?}", test2);
    println!("{:?}", test3);

    Ok(())
}
