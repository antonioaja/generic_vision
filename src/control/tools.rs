use super::colorspaces::{self, Pixel, HSV};
use image::DynamicImage;
use rgb::RGB;
use std::time::Instant;

#[derive(Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Point<N> {
    pub x: N,
    pub y: N,
}

#[derive(Clone, Debug, Default, PartialEq)]
/// Controls position adjustment parameters
pub struct PositionAdjust {
    width: u32,
    length: u32,
    top_left_corner: [u32; 2],
    base_image: DynamicImage,
}
impl PositionAdjust {
    /// Returns a struct with all fields zero
    pub fn new(w: u32, h: u32) -> PositionAdjust {
        Self {
            width: w,
            length: h,
            top_left_corner: [0, 0],
            base_image: DynamicImage::new_luma8(w, h),
        }
    }

    /// Sets all parameters to 0
    pub fn initialize(&mut self, w: u32, h: u32) {
        self.width = 0;
        self.length = 0;
        self.top_left_corner = [0, 0];
        self.base_image = DynamicImage::new_luma8(w, h);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
/// Controls color area tool
pub struct ColorArea {
    width: u32,
    length: u32,
    top_left_corner: Point<u32>,
    hue: [f64; 2],
    saturation: [f64; 2],
    value: [f64; 2],
    id: u8,
    name: &'static str,
}
impl ColorArea {
    /// Returns a struct with all fields zero
    fn new() -> ColorArea {
        Self {
            width: 3839,
            length: 2159,
            top_left_corner: Point { x: 0, y: 0 },
            hue: [0.0, 69.0],
            saturation: [0.0, 1.0],
            value: [0.0, 1.0],
            id: 0,
            name: "",
        }
    }

    /// Sets all parameters to 0
    pub fn initialize(&mut self) {
        self.width = 0;
        self.length = 0;
        self.top_left_corner = Point { x: 0, y: 0 };
        self.hue = [0.0, 0.0];
        self.saturation = [0.0, 0.0];
        self.value = [0.0, 0.0];
        self.id = 0;
    }

    /// Returns a percentage match to the set parameters
    pub fn check(&self, input: &[RGB<u8>], width: u32, height: u32) -> f64 {
        let now = Instant::now();

        let source: Vec<HSV<f64>> = input
            .iter()
            .map(|x| colorspaces::HSV::from_rgb8(*x))
            .collect();

        println!("{} ms", now.elapsed().as_millis());

        let mut count: u32 = 0;
        let total_area = self.length * self.width;

        for y in self.top_left_corner.y..=(self.top_left_corner.y + self.length) {
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

        println!("{:?}", (count as f64) / (total_area as f64));

        // let total_area = self.length * self.width;
        // let a = self.top_left_corner.y;
        // let b = self.top_left_corner.x;
        // let c = width - b - self.width;
        // let d = height - a - self.length;

        println!("{} ms", now.elapsed().as_millis());

        100.00
    }
}

impl Default for ColorArea {
    fn default() -> Self {
        Self::new()
    }
}
