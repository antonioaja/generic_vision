use anyhow::*;
use clap::Parser;
use std::ffi::OsStr;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(version, about = "A generic vision program.", long_about = None)]
pub struct Args {
    /// Name of input file
    #[clap(short, long, value_parser)]
    pub input_file: String,

    /// Name of output file
    #[clap(short, long, value_parser, default_value = "")]
    pub output_file: String,
}

pub fn get_extension(filename: &str) -> Result<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .context(format!("Could not find extension for {}", filename))
}
