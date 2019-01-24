extern crate image;
#[macro_use]
extern crate serde_derive;

mod colors;
mod analyze;

use std::collections::HashMap;
use image::DynamicImage;
pub use colors::Color;
pub use analyze::Prediction;

pub fn parse(color_map: &mut HashMap<String, Color>) {
    colors::parse(color_map)
}

pub fn predict(
    image: DynamicImage,
    colors: HashMap<String, Color>,
    predictions: &mut Vec<Prediction>
) {
    analyze::predict(image, colors, predictions)
}
