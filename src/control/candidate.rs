use image::DynamicImage;
use uuid::Uuid;

#[derive(Clone)]
/// A candidate image to compare to model
pub struct Candidate {
    image: DynamicImage,
    curl: f64,
    offset: [u32; 2],
    scores: Vec<f64>,
    des_uuid: Uuid,
}
