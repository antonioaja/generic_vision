use crate::control::tools::ColorArea;
use crate::control::tools::PositionAdjust;

#[derive(Clone)]
/// An image to compare against
pub struct Model {
    pos_adjust: PositionAdjust,
    color_tools: Vec<ColorArea>,
}

impl Model {
    /// Returns Model object with all fields set to 0
    pub fn new() -> Model {
        Self {
            pos_adjust: PositionAdjust::new(),
            color_tools: vec![],
        }
    }

    pub fn find_curl(&mut self, ref_model: Model){
        
    }
}
