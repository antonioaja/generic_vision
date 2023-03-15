pub mod control;
pub mod misc;
pub mod settings;

use anyhow::*;
use std::time::Instant;

fn main() -> Result<()> {
    let now = Instant::now();

    let _setting = settings::Settings::get_model_info("PROGRAM_1903.toml".to_string()).unwrap();

    println!("{} ms", now.elapsed().as_millis());

    Ok(())
}
