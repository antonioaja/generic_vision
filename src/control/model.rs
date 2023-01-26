use anyhow::*;
use image::DynamicImage;
use image_compare::*;
use imageproc::geometric_transformations::rotate_about_center;
use imageproc::geometric_transformations::translate;
use imageproc::geometric_transformations::Interpolation;
use std::time::Instant;
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

    /// Finds the angle offset compared to the master image
    pub fn find_curl(&mut self, candidate: DynamicImage, curl: &mut f64) -> Result<()> {
        let now = Instant::now();

        let detection_candidate: DynamicImage =
            edge_detection::canny(candidate.to_luma8(), 2.0, 0.2, 0.01).as_image();
        let detection_master: DynamicImage =
            edge_detection::canny(image::open(&self.image_path)?.to_luma8(), 2.0, 0.2, 0.01)
                .as_image();

        let mut mini_y: f64 = 0.0;
        let mut mini_x = 0;

        for i in 0..=360 {
            let rotated = rotate_about_center(
                &detection_candidate.to_luma8(),
                (i as f64 * ONE_DEGREE) as f32,
                Interpolation::Bicubic,
                image::Luma([0]),
            );

            let result = image_compare::gray_similarity_histogram(
                Metric::Correlation,
                &detection_master.to_luma8(),
                &rotated,
            )
            .context("Could not compare")?;

            if i == 0 || result < mini_y {
                mini_y = result;
                mini_x = i;
            }
        }

        *curl = mini_x as f64;
        println!("{} ms", now.elapsed().as_millis());

        Ok(())
    }

    /// Finds the position offset compared to the master image
    pub fn find_offset(&mut self, candidate: DynamicImage, curl: f64) -> Result<()> {
        let detection_candidate: DynamicImage =
            edge_detection::canny(candidate.to_luma8(), 2.0, 0.2, 0.01).as_image();
        let detection_master: DynamicImage = edge_detection::canny(
            image::open(&self.image_path)
                .context(format!("Could not open {}", &self.image_path))?
                .to_luma8(),
            2.0,
            0.2,
            0.01,
        )
        .as_image();

        let rotated = rotate_about_center(
            &detection_candidate.to_luma8(),
            (curl * ONE_DEGREE) as f32,
            Interpolation::Bicubic,
            image::Luma([0]),
        );

        let mut mini_y: f64 = 0.0;
        let mut mini_x: i32 = 0;

        for i in 1..=(detection_master.width() / 2) {
            let shifted = translate(&rotated, (i as i32, 0));

            let result = image_compare::gray_similarity_histogram(
                Metric::Intersection,
                &detection_master.to_luma8(),
                &shifted,
            )
            .context("Could not compare")?;

            if i == 1 || result < mini_y {
                mini_y = result;
                mini_x = i as i32;
            }

            let shifted = translate(&rotated, (-(i as i32), 0));

            let result = image_compare::gray_similarity_histogram(
                Metric::Intersection,
                &detection_master.to_luma8(),
                &shifted,
            )
            .context("Could not compare")?;

            if result < mini_y {
                mini_y = result;
                mini_x = -(i as i32);
            }
        }

        let x = mini_x;

        for i in 1..=(detection_master.height() / 2) {
            let shifted = translate(&rotated, (x, i as i32));

            let result = image_compare::gray_similarity_histogram(
                Metric::Intersection,
                &detection_master.to_luma8(),
                &shifted,
            )
            .context("Could not compare")?;

            if i == 1 || result < mini_y {
                mini_y = result;
                mini_x = i as i32;
            }

            let shifted = translate(&rotated, (x, -(i as i32)));

            let result = image_compare::gray_similarity_histogram(
                Metric::Intersection,
                &detection_master.to_luma8(),
                &shifted,
            )
            .context("Could not compare")?;

            if result < mini_y {
                mini_y = result;
                mini_x = -(i as i32);
            }
        }

        let y = mini_x;

        println!("{},{}", x, y);

        translate(&rotated, (x, y)).save("shift.png")?;

        Ok(())
    }
}
