pub mod control;
pub mod misc;
pub mod settings;

use altered_perception::HSV;
use anyhow::*;
use clap::Parser;
use image::EncodableLayout;
use rayon::prelude::*;
use rgb::FromSlice;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

#[derive(Parser, Debug)]
#[clap(
    author = "Antonio Aguilar",
    version,
    about = "Very basic vision program"
)]
pub struct Args {
    /// The input image
    #[clap(short, long, value_parser)]
    pub input: String,

    /// Settings file location
    #[clap(short, long, value_parser)]
    pub settings_file: String,
}

fn main() -> Result<()> {
    let now = Instant::now();

    let args: Args = Args::parse();
    let input = &args.input;
    let settings_file = args.settings_file;

    let setting = settings::Settings::get_model_info(settings_file)
        .context("Could not find settings file")?;

    let candidate_image = image::open(input).context(format!("Could not open {}", input))?;

    let candidate_hsv: Vec<HSV<f64>> = candidate_image
        .clone()
        .into_rgb8()
        .as_bytes()
        .as_rgb()
        .par_iter()
        .map(|x| HSV::from_rgb(*x))
        .collect();

    let mut file =
        File::create(format!("{}.txt", input)).context("Could not create output file.")?;

    let outs = format!(
        "{},{},{}",
        input,
        setting.color_tools[0]
            .check(
                candidate_hsv.clone(),
                candidate_image.width(),
                candidate_image.height()
            )
            .to_string(),
        now.elapsed().as_millis()
    );

    file.write_all(outs.as_bytes())
        .context("Could not write to output file.")?;

    Ok(())
}
