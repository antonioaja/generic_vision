use super::colorspaces::{self, Pixel, HSV};
use image::DynamicImage;
use rgb::RGB;

#[derive(Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Point<N> {
    pub x: N,
    pub y: N,
}

#[derive(Clone, Debug, Default, PartialEq)]
/// Controls position adjustment parameters
pub struct PositionAdjust {
    width: u32,
    height: u32,
    top_left_corner: [u32; 2],
    base_image: DynamicImage,
}
impl PositionAdjust {
    /// Returns a struct with all fields zero
    pub fn new(w: u32, h: u32) -> PositionAdjust {
        Self {
            width: w,
            height: h,
            top_left_corner: [0, 0],
            base_image: DynamicImage::new_luma8(w, h),
        }
    }

    /// Sets all parameters to 0
    pub fn initialize(&mut self, w: u32, h: u32) {
        self.width = 0;
        self.height = 0;
        self.top_left_corner = [0, 0];
        self.base_image = DynamicImage::new_luma8(w, h);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
/// Controls color area tool
pub struct ColorArea {
    width: u32,
    height: u32,
    top_left_corner: Point<u32>,
    hue: [f64; 2],
    saturation: [f64; 2],
    value: [f64; 2],
    id: u8,
    name: &'static str,
}
impl ColorArea {
    ///  Creates Struct given parameters
    pub fn new(
        width: u32,
        height: u32,
        top_left_corner: Point<u32>,
        hue: [f64; 2],
        saturation: [f64; 2],
        value: [f64; 2],
        id: u8,
        name: &'static str,
    ) -> ColorArea {
        Self {
            width,
            height,
            top_left_corner,
            hue,
            saturation,
            value,
            id,
            name,
        }
    }

    // /// Sets all parameters to 0
    // pub fn initialize(&mut self) {
    //     self.width = 0;
    //     self.height = 0;
    //     self.top_left_corner = Point { x: 0, y: 0 };
    //     self.hue = [0.0, 0.0];
    //     self.saturation = [0.0, 0.0];
    //     self.value = [0.0, 0.0];
    //     self.id = 0;
    // }

    /// Returns a percentage match to the set parameters
    pub fn check(&self, input: &[RGB<u8>], width: u32) -> f64 {
        let source: Vec<HSV<f64>> = input
            .iter()
            .map(|x| colorspaces::HSV::from_rgb8(*x))
            .collect();

        let mut count: u32 = 0;
        let total_area = self.height * self.width;

        for y in self.top_left_corner.y..=(self.top_left_corner.y + self.height) {
            for x in self.top_left_corner.x..=(self.top_left_corner.x + self.width) {
                let pixel = source.get_pixel(x, y, width);

                if pixel.h > self.hue[1] || pixel.h < self.hue[0] {
                } else if pixel.s > self.saturation[1] || pixel.s < self.saturation[0] {
                } else if pixel.v > self.value[1] || pixel.v < self.value[0] {
                } else {
                    count += 1;
                }
            }
        }

        (count / total_area).into()
    }
}
