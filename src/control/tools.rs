use image::DynamicImage;
use rgb::FromSlice;

use super::colorspaces;

#[derive(Clone)]
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
            width: 0,
            length: 0,
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

#[derive(Copy, Clone)]
/// Controls color area tool
pub struct ColorArea {
    width: u32,
    length: u32,
    top_left_corner: [u32; 2],
    hue: [u32; 2],
    saturation: [u32; 2],
    value: [u32; 2],
    id: u32,
}
impl ColorArea {
    /// Returns a struct with all fields zero
    pub fn new() -> ColorArea {
        Self {
            width: 0,
            length: 0,
            top_left_corner: [0, 0],
            hue: [0, 0],
            saturation: [0, 0],
            value: [0, 0],
            id: 0,
        }
    }

    /// Sets all parameters to 0
    pub fn initialize(&mut self) {
        self.width = 0;
        self.length = 0;
        self.top_left_corner = [0, 0];
        self.hue = [0, 0];
        self.saturation = [0, 0];
        self.value = [0, 0];
        self.id = 0;
    }

    /// Returns a percentage match to the set parameters
    pub fn check(&self, input: DynamicImage) -> f64{
        let source = input.as_bytes().as_rgb();

        let hsv_rep = colorspaces::HSV::from_rgb(source[0]);

        println!("{:?}", source[0]);
        println!("{:?}", hsv_rep);
        

        100.00
    }
}

impl Default for ColorArea {
    fn default() -> Self {
        Self::new()
    }
}