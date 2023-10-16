use altered_perception::Luma;
use anyhow::Context;
use image::EncodableLayout;
use rayon::prelude::*;
use rgb::FromSlice;
use serde_derive::{Deserialize, Serialize};
use std::{fs, str::FromStr};
use toml::value::*;
use uuid::Uuid;

use crate::{
    control::{
        model::Model,
        tools::{ColorArea, PositionAdjust},
    },
    misc::helpers::{Dimensions, Identification, Point, Range},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PartialOrd)]
pub struct Settings {
    name: String,
    uuid: String,
    date_created: Datetime,
    last_edited: Datetime,
    path_to_master: String,
    path_to_edges: String,
    pos_adjust: PositionAdjustInterpretation,
    color: Vec<ColorAreaInterpretation>,
}
impl Settings {
    pub fn get_model_info(file_name: String) -> Option<Model> {
        let contents = fs::read_to_string(file_name).unwrap_or("".to_string());
        let opt_settings: Option<Settings> = toml::from_str(&contents)
            .context("Could not read settings into memory")
            .ok();

        if opt_settings.is_none() {
            None
        } else {
            let settings = opt_settings.unwrap();

            let mut color_tools: Vec<ColorArea> = vec![];
            for i in 0..settings.color.len() {
                color_tools.push(ColorArea::new(
                    Dimensions {
                        width: settings.color[i].width,
                        height: settings.color[i].length,
                    },
                    Point {
                        x: settings.color[i].corner[0],
                        y: settings.color[i].corner[1],
                    },
                    Range {
                        lower: settings.color[i].hue[0],
                        upper: settings.color[i].hue[1],
                    },
                    Range {
                        lower: settings.color[i].saturation[0],
                        upper: settings.color[i].saturation[1],
                    },
                    Range {
                        lower: settings.color[i].value[0],
                        upper: settings.color[i].value[1],
                    },
                    Identification {
                        name: settings.color[i].name.clone(),
                        id: settings.color[i].id,
                    },
                ))
            }

            return Some(Model::new(
                Identification {
                    name: settings.name.clone(),
                    id: Uuid::from_str(&settings.uuid).unwrap(),
                },
                PositionAdjust::new(
                    Dimensions {
                        width: settings.pos_adjust.width,
                        height: settings.pos_adjust.length,
                    },
                    Point {
                        x: settings.pos_adjust.corner[0],
                        y: settings.pos_adjust.corner[1],
                    },
                    image::open(settings.path_to_edges.clone())
                        .unwrap()
                        .into_rgb8()
                        .as_bytes()
                        .as_rgb()
                        .par_iter()
                        .map(|x| Luma::from_rgb(*x))
                        .collect(),
                ),
                color_tools,
                settings.path_to_master.clone(),
            ));
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct PositionAdjustInterpretation {
    pub width: u32,
    pub length: u32,
    pub corner: Vec<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PartialOrd)]
struct ColorAreaInterpretation {
    pub width: u32,
    pub length: u32,
    pub corner: Vec<u32>,
    pub hue: Vec<f64>,
    pub value: Vec<f64>,
    pub saturation: Vec<f64>,
    pub id: u8,
    pub name: String,
}
