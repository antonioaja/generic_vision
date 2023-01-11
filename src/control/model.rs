use anyhow::*;
use image::DynamicImage;
use image_compare::Algorithm;
use imageproc::geometric_transformations::rotate_about_center;
use imageproc::geometric_transformations::Interpolation;
use uuid::Uuid;

use crate::control::tools::ColorArea;
use crate::control::tools::PositionAdjust;

/// One degree in radians
const ONE_DEGREE: f64 = std::f64::consts::PI / 180.0;

#[derive(Clone)]
/// An image to compare against
pub struct Model {
    pos_adjust: PositionAdjust,
    color_tools: Vec<ColorArea>,
    uuid: Uuid,
    name: String,
    image_path: String,
}

impl Model {
    /// Returns blank Model object with random uuid
    pub fn new(name: String, image_path: String, w: u32, h: u32) -> Model {
        Self {
            pos_adjust: PositionAdjust::new(w, h),
            color_tools: vec![],
            uuid: Uuid::new_v4(),
            name,
            image_path,
        }
    }

    pub fn find_curl(&mut self, candidate: DynamicImage, _curl: &mut f64) -> Result<()> {
        let detection_candidate: DynamicImage =
            edge_detection::canny(candidate.to_luma8(), 2.0, 0.2, 0.01).as_image();
        let detection_master: DynamicImage = edge_detection::canny(
            image::open(self.image_path.to_string())?.to_luma8(),
            2.0,
            0.2,
            0.01,
        )
        .as_image();

        let mut mini_y: f64 = 0.0;
        let mut mini_x = 0;

        for i in 0..=360 {
            println!("{}", i);
            let rotated = rotate_about_center(
                &detection_candidate.to_luma8(),
                (i as f64 * ONE_DEGREE) as f32,
                Interpolation::Nearest,
                image::Luma([0]),
            );

            let result = image_compare::gray_similarity_structure(
                &Algorithm::MSSIMSimple,
                &detection_master.to_luma8(),
                &rotated,
            )
            .context("Could not compare")?;

            if i == 0 || result.score < mini_y {
                mini_y = result.score;
                mini_x = i;
            }

            println!("{},{}", mini_x, mini_y);
        }

        Ok(())
    }
}
