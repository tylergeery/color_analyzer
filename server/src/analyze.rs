use std::collections::HashMap;
use image::{RgbaImage, imageops, Rgba};
use colors::Color;
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
pub struct Prediction {
    pub hex: String,
    pub name: String,
    pub score: f64
}

pub fn predict(
    rgba_image: RgbaImage,
    colors: &HashMap<String, Color>,
    predictions: &mut Vec<Prediction>
) {
    let mut results: HashMap<String, u64> = HashMap::new();
    let pixels = rgba_image.pixels();
    let mut pixel_count = 0;

    for pix in pixels {
        let winner = get_closest_color(*pix, &colors);
        pixel_count += 1;
        let s = winner.to_string();
        let val = results.entry(s.clone()).or_insert(0);
        *val += 1;
    }

    for color in results.keys() {
        predictions.push(Prediction {
            hex: colors.get(color).unwrap().hex.clone(),
            name: color.clone(),
            score: (*results.get(color).unwrap() as f64) / f64::from(pixel_count)
        });
    }

    predictions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
}

pub fn center_image(mut img: RgbaImage) -> RgbaImage {
    let dimensions = img.dimensions();
    let x_offset: u32 = dimensions.0 / 4;
    let y_offset: u32 = dimensions.1 / 4;
    let x_length: u32 = dimensions.0 - (x_offset * 2);
    let y_length: u32 = dimensions.1 - (y_offset * 2);

    imageops::crop(&mut img, x_offset, y_offset, x_length, y_length).to_image()
}

pub fn predict_cluster(
    rgba_image: RgbaImage,
    colors: &HashMap<String, Color>,
    predictions: &mut Vec<Prediction>
) {
    let mut results: HashMap<String, u64> = HashMap::new();
    let pixels = rgba_image.pixels();
    let mut winners: Vec<String> = Vec::new();
    let mut total_value = 0;
    let mut pixel_count = 0;

    for pix in pixels {
        let winner = get_closest_color(*pix, &colors);
        let s = winner.to_string();
        results.entry(s.clone()).or_insert(0);
        winners.push(s.clone());
    }

    for winner in &winners {
        let nc = get_neighboring_count(pixel_count, &winners, rgba_image.dimensions());
        let val = results.entry(winner.to_string()).or_insert(0);
        let count = 1 + nc;

        *val += count;
        total_value += count;
        pixel_count += 1;
    }

    for color in results.keys() {
        predictions.push(Prediction {
            hex: colors.get(color).unwrap().hex.clone(),
            name: color.clone(),
            score: (*results.get(color).unwrap() as f64) / (total_value as f64)
        });
    }

    predictions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
}

fn dist(base: u64, supplied: u64) -> u64 {
    if base > supplied { base - supplied } else { supplied - base }
}

fn get_closest_color(pix: Rgba<u8>, colors: &HashMap<String, Color>) -> String
{
    let mut curr: f64 = 1000.0;
    let red = u64::from(pix.data[0]);
    let green = u64::from(pix.data[1]);
    let blue = u64::from(pix.data[2]);
    let mut winner = String::new();

    for key in colors.keys() {
        let color = colors.get(key).unwrap();
        let euclidean = (dist(red, color.rgb[0])^2) + (dist(green, color.rgb[1])^2) + (dist(blue, color.rgb[2])^2);
        let value = f64::sqrt(euclidean as f64);

        if value < curr {
            curr = value;
            winner = key.to_string();
        }
    }

    winner
}

fn get_neighboring_count(
    i: usize,
    winners: &[String],
    dimensions: (u32, u32)
) -> u64 {
    let mut count = 0;

    if ((i as u32) % dimensions.0) != 0 && winners[i] == winners[i-1] {
        count += 1;
    }

    if (i as u32) >= dimensions.1 && winners[i] == winners[i - (dimensions.1 as usize)] {
        count += 1;
    }

    if ((i as u32) % dimensions.0) != (dimensions.0 - 1) && winners[i] == winners[i+1] {
        count += 1;
    }

    if (i as u32) < (dimensions.0 * (dimensions.1 - 1)) && winners[i] == winners[i + (dimensions.0 as usize)] {
        count += 1;
    }

    count
}
