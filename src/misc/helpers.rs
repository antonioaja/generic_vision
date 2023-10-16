use clap::Parser;
use serde_derive::{Deserialize, Serialize};
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

/// Returns extension (if available) an input string
/// Example: "test.png" => "png"
pub fn get_extension(filename: &str) -> &str {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("")
}

#[derive(
    Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash, Deserialize, Serialize,
)]
/// A 2d point in space
pub struct Point<N> {
    pub x: N,
    pub y: N,
}

#[derive(
    Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash, Deserialize, Serialize,
)]
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

#[derive(
    Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash, Deserialize, Serialize,
)]
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

#[derive(Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Identification<T> {
    pub name: String,
    pub id: T,
}

#[derive(
    Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash, Deserialize, Serialize,
)]
/// A rectangular object
pub struct Rectangle<N> {
    pub left: N,
    pub right: N,
    pub top: N,
    pub bottom: N,
}
impl<T: std::ops::Add<Output = T> + Copy> Rectangle<T> {
    pub fn create_rectangle(top_left: Point<T>, bottom_right: Point<T>) -> Self {
        Self {
            left: top_left.x,
            right: top_left.x + bottom_right.x,
            top: top_left.y,
            bottom: top_left.y + bottom_right.y,
        }
    }

    // Returns overlapped area of two rectangles
    // pub fn overlap_area(&self, rect2: Rectangle<T>) -> f64 {
    //     let x_overlap = cmp::max(0.0, cmp::min(self.right, rect2.right));

    //     0.0
    // }
}
