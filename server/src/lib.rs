extern crate image;
#[macro_use]
extern crate serde_derive;
extern crate fnv;

use self::fnv::FnvHashMap;

mod colors;
mod analyze;

use std::collections::HashMap;
use image::{RgbaImage};
pub use colors::Color;
pub use analyze::Prediction;

pub fn parse() -> FnvHashMap<String, Color> {
    colors::parse()
}

pub fn predict<S: ::std::hash::BuildHasher>(
    image: RgbaImage,
    colors: HashMap<String, Color, S>,
    predictions: &mut Vec<Prediction>
) {
    analyze::predict(image, &colors, predictions)
}

pub fn predict_cluster<S: ::std::hash::BuildHasher>(
    image: RgbaImage,
    colors: HashMap<String, Color, S>,
    predictions: &mut Vec<Prediction>
) {
    analyze::predict_cluster(image, &colors, predictions)
}

pub fn center_image(image: RgbaImage) -> RgbaImage {
    analyze::center_image(image)
}
