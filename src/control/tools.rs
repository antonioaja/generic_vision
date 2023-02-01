use super::colorspaces::{self, Pixel, HSV};
use crate::misc::helpers::*;
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
    hue: Range<f64>,
    saturation: Range<f64>,
    value: Range<f64>,
    identification: Identification<u8>,
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

                if self.hue.within_range_inclusive(pixel.h)
                    && self.saturation.within_range_inclusive(pixel.s)
                    && self.value.within_range_inclusive(pixel.v)
                {
                    count += 1;
                }
            }
        }

        f64::round(count as f64 / total_area as f64 * 100.0) / 100.0
    }

    ///  Creates Struct given parameters
    pub fn new(
        dimension: Dimensions<u32>,
        top_left_corner: Point<u32>,
        hue: Range<f64>,
        saturation: Range<f64>,
        value: Range<f64>,
        identification: Identification<u8>,
    ) -> ColorArea {
        Self {
            dimension,
            top_left_corner,
            hue,
            saturation,
            value,
            identification,
        }
    }
}
