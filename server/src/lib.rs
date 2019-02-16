extern crate image;
#[macro_use]
extern crate serde_derive;

mod colors;
mod analyze;

use std::collections::HashMap;
use image::{RgbaImage};
pub use colors::Color;
pub use analyze::Prediction;

pub fn parse() -> HashMap<String, Color> {
    colors::parse()
}

pub fn predict(
    image: RgbaImage,
    colors: HashMap<String, Color>,
    predictions: &mut Vec<Prediction>
) {
    analyze::predict(image, &colors, predictions)
}

pub fn predict_cluster(
    image: RgbaImage,
    colors: HashMap<String, Color>,
    predictions: &mut Vec<Prediction>
) {
    analyze::predict_cluster(image, &colors, predictions)
}

pub fn center_image(image: RgbaImage) -> RgbaImage {
    analyze::center_image(image)
}
