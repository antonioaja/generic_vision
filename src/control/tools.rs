use super::colorspaces::{self, Pixel, HSV};
use image::DynamicImage;
use rgb::RGB;

#[derive(Clone, Debug, Default, PartialEq)]
/// Controls position adjustment parameters
pub struct PositionAdjust {
    dimension: Dimensions<u32>,
    top_left_corner: Point<u32>,
    base_image: DynamicImage,
}
impl PositionAdjust {
    /// Returns a struct with all fields zero
    pub fn new(dimension: Dimensions<u32>, top_left_corner: Point<u32>) -> PositionAdjust {
        Self {
            dimension,
            top_left_corner,
            base_image: DynamicImage::new_luma8(dimension.width, dimension.height),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
/// Controls color area tool
pub struct ColorArea {
    dimension: Dimensions<u32>,
    top_left_corner: Point<u32>,
    hue: [f64; 2],
    saturation: [f64; 2],
    value: [f64; 2],
    id: u8,
    name: &'static str,
}
impl ColorArea {
    /// Returns a percentage match to the set parameters
    pub fn check(&self, input: &[RGB<u8>], width: u32) -> f64 {
        let source: Vec<HSV<f64>> = input
            .iter()
            .map(|x| colorspaces::HSV::from_rgb8(*x))
            .collect();

        let mut count: u32 = 0;
        let total_area = self.dimension.height * self.dimension.width;

        for y in self.top_left_corner.y..=(self.top_left_corner.y + self.dimension.height) {
            for x in self.top_left_corner.x..=(self.top_left_corner.x + self.dimension.width) {
                let pixel = source.get_pixel(x, y, width);

                if pixel.h > self.hue[1]
                    || pixel.h < self.hue[0]
                    || pixel.s > self.saturation[1]
                    || pixel.s < self.saturation[0]
                    || pixel.v > self.value[1]
                    || pixel.v < self.value[0]
                {
                } else {
                    count += 1;
                }
            }
        }

        count as f64 / total_area as f64
    }

    ///  Creates Struct given parameters
    pub fn new(
        dimension: Dimensions<u32>,
        top_left_corner: Point<u32>,
        hue: [f64; 2],
        saturation: [f64; 2],
        value: [f64; 2],
        id: u8,
        name: &'static str,
    ) -> ColorArea {
        Self {
            dimension,
            top_left_corner,
            hue,
            saturation,
            value,
            id,
            name,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
/// A 2d point in space
pub struct Point<N> {
    pub x: N,
    pub y: N,
}

#[derive(Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
/// Dimensions of 2d object
pub struct Dimensions<N> {
    pub width: N,
    pub height: N,
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
