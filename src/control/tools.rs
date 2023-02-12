use crate::matrix_slice_2d_impl;
use crate::misc::helpers::*;
use crate::misc::linear_algebra::MatrixSlice2d;
use altered_perception::{Luma, HSV};
use image::{DynamicImage, EncodableLayout, RgbImage};
use rayon::prelude::*;
use rgb::RGB;
use rgb::{ComponentBytes, FromSlice};
use serde_derive::Serialize;

matrix_slice_2d_impl!(Luma);
matrix_slice_2d_impl!(RGB);
matrix_slice_2d_impl!(HSV);

#[derive(Clone, Debug, PartialEq, PartialOrd, Default, Eq, Ord, Hash)]
/// Controls position adjustment parameters
pub struct PositionAdjust {
    dimension: Dimensions<u32>,
    top_left_corner: Point<u32>,
    edge_image: Vec<Luma<u8>>,
}
impl PositionAdjust {
    /// Returns a struct with all fields zero
    pub fn new(
        dimension: Dimensions<u32>,
        top_left_corner: Point<u32>,
        edge_image: Vec<Luma<u8>>,
    ) -> PositionAdjust {
        Self {
            dimension,
            top_left_corner,
            edge_image,
        }
    }

    pub fn find_curl(des_image: Vec<RGB<u8>>, des_dimensions: Dimensions<u32>) -> f64 {
        let initial_conversion = DynamicImage::ImageRgb8(
            RgbImage::from_raw(
                des_dimensions.width,
                des_dimensions.height,
                des_image.as_bytes().to_vec(),
            )
            .unwrap(),
        )
        .into_luma8();

        let lumaed: Vec<Luma<u8>> = edge_detection::canny(initial_conversion, 1.2, 0.2, 0.01)
            .as_image()
            .into_rgb8()
            .as_bytes()
            .as_rgb()
            .par_iter()
            .map(|x| Luma::from_rgb(*x))
            .collect();

        println!("{:?}", lumaed);

        0.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default, Serialize)]
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
    pub fn check(&self, input: Vec<HSV<f64>>, width: u32) -> f64 {
        let mut count: u32 = 0;
        let total_area = self.dimension.height * self.dimension.width;

        (self.top_left_corner.y..=(self.top_left_corner.y + self.dimension.height)).for_each(|y| {
            (self.top_left_corner.x..=(self.top_left_corner.x + self.dimension.width)).for_each(
                |x| {
                    let pixel = input.interpret_position(x, y, width, 0);

                    if self.hue.within_range_inclusive(pixel.h)
                        && self.saturation.within_range_inclusive(pixel.s)
                        && self.value.within_range_inclusive(pixel.v)
                    {
                        count += 1;
                    }
                },
            );
        });

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
