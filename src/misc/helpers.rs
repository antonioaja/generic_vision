use anyhow::*;
use clap::Parser;
use std::ffi::OsStr;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(version, about = "A generic vision program.", long_about = None)]
pub struct Args {
    /// Name of input file
    #[clap(short, long, value_parser, default_value = "test.png")]
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

#[derive(Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
/// A 2d point in space
pub struct Point<N> {
    pub x: N,
    pub y: N,
}

#[derive(Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
/// Dimensions of a 2d object
pub struct Dimensions<N> {
    pub width: N,
    pub height: N,
}
impl<T> Dimensions<T>
where
    T: std::ops::Mul<Output = T>,
    T: Copy,
{
    /// Returns area of dimensions
    pub fn area(&self) -> T {
        self.width * self.height
    }
}

#[derive(Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
/// A ranged object
pub struct Range<N> {
    pub lower: N,
    pub upper: N,
}
impl<T> Range<T>
where
    T: std::cmp::PartialOrd,
{
    /// Returns whether given value is within a given range (inclusive)
    pub fn within_range_inclusive(&self, value: T) -> bool {
        if self.upper > self.lower {
            if value <= self.upper && value >= self.lower {
                return true;
            }
        } else if value >= self.upper && value <= self.lower {
            return true;
        }

        false
    }

    /// Returns whether given value is within a given range (exclusive)
    pub fn within_range_exclusive(&self, value: T) -> bool {
        if self.upper > self.lower {
            if value < self.upper && value > self.lower {
                return true;
            }
        } else if value > self.upper && value < self.lower {
            return true;
        }

        false
    }
}

#[derive(Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Identification<T> {
    pub name: &'static str,
    pub id: T,
}
