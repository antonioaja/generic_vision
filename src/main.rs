pub mod control;
pub mod misc;
pub mod settings;

use anyhow::*;
use std::time::Instant;

fn main() -> Result<()> {
    let setting = settings::Settings::get_model_info("PROGRAM_1903.toml".to_string()).unwrap();

    let now = Instant::now();

    println!("{} ms", now.elapsed().as_millis());

    Ok(())
}
