use image::EncodableLayout;
use rgb::{FromSlice, RGB};
use uuid::Uuid;
use serde_derive::{Deserialize, Serialize};

use crate::control::tools::*;
use crate::misc::helpers::*;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
/// An image to compare against
pub struct Model {
    pos_adjust: PositionAdjust,
    color_tools: Vec<ColorArea>,
    identification: Identification<Uuid>,
    image_data: Vec<RGB<u8>>,
}

impl Model {
    /// Creates a Model
    pub fn new(
        identification: Identification<Uuid>,
        pos_adjust: PositionAdjust,
        color_tools: Vec<ColorArea>,
        master_location: String,
    ) -> Model {
        Self {
            pos_adjust,
            color_tools,
            identification,
            image_data: image::open(master_location)
                .unwrap()
                .into_rgb8()
                .as_bytes()
                .as_rgb()
                .to_vec(),
        }
    }

    pub fn compare() {}
}
