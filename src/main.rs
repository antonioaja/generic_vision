pub mod control;
pub mod misc;

use anyhow::*;
// use clap::Parser;
// use imageproc::geometric_transformations::rotate_about_center;
// use imageproc::geometric_transformations::Interpolation;

// use crate::misc::helpers::get_extension;
// use crate::misc::helpers::Args;

fn main() -> Result<()> {
    let mut test_model = control::model::Model::new("ope".to_string(), "test.png".to_string(), 10, 10);

    let ope = image::open("test_rotate.png")?;

    let mut var: f64 = 0.0;

    test_model.find_curl(ope, &mut var)?;

    Ok(())
}
