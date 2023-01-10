use anyhow::*;
use dssim::Dssim;
use image::DynamicImage;
use imageproc::geometric_transformations::rotate_about_center;
use imageproc::geometric_transformations::Interpolation;
use tempdir::TempDir;
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

    pub fn find_curl(&mut self, candidate: DynamicImage, curl: &mut f64) -> Result<()> {
        let tmp_dir = TempDir::new("ope").context("Could not create temporary directory")?;
        let file_path = tmp_dir.path().join("temp.png");

        let detection = edge_detection::canny(candidate.to_luma8(), 2.0, 0.2, 0.01).as_image();

        detection.save(&file_path).context(format!(
            "Could not save edge detection image to {}",
            &file_path.display()
        ))?;

        let attr = Dssim::new();
        let master = dssim::load_image(&attr, &self.image_path).context(format!(
            "Could not load {} for image comparison",
            &self.image_path
        ))?;

        let file_path_rot = tmp_dir.path().join("rotate.png");

        println!("{}", &file_path.display());

        let mut mini_x: i32 = 0;
        let mut mini_y: f64 = 0.0;

        for i in 0..=360 {

            println!("{}", i);
            rotate_about_center(
                &detection.to_rgb8(),
                (i as f64 * ONE_DEGREE) as f32,
                Interpolation::Nearest,
                image::Rgb([0, 0, 0]),
            )
            .save(&file_path_rot)
            .context("Could not save rotated image")?;

            let contender = dssim::load_image(&attr, &file_path).context(format!(
                "Could not load {} for image comparison",
                &file_path_rot.display()
            ))?;

            let (diff, _) = attr.compare(&master, &contender);

            if i == 0 || diff < mini_y {
                mini_y = diff.into();
                mini_x = i;
            }

            //println!("{},{}", mini_x, mini_y);
        }

        tmp_dir
            .close()
            .context("Could not close temporary directory")?;

        Ok(())
    }
}
