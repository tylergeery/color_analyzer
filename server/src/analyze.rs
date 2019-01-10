use std::collections::HashMap;
use image::DynamicImage;

#[derive(Serialize, Deserialize, Debug)]
pub struct Prediction {
    name: String,
    score: f64
}

pub fn predict(
    image: DynamicImage,
    colors: HashMap<String, Vec<u64>>,
    predictions: &mut Vec<Prediction>
) {
    let mut results: HashMap<String, u64> = HashMap::new();
    let rgba_image = image.to_rgba();
    let pixels = rgba_image.pixels();
    let mut pixel_count = 0;
    let mut winner = String::new();

    for pix in pixels {
        let mut curr: f64 = 1000.0;
        let red = u64::from(pix.data[0]);
        let green = u64::from(pix.data[1]);
        let blue = u64::from(pix.data[2]);

        for key in colors.keys() {
            let color = colors.get(key).unwrap();
            let euclidean = (dist(red, color[0])^2) + (dist(green, color[1])^2) + (dist(blue, color[2])^2);
            let value = f64::sqrt(euclidean as f64);

            if value < curr {
                curr = value;
                winner = key.to_string();
            }
        }

        pixel_count += 1;
        let s = winner.to_string();
        let val = results.entry(s).or_insert(0);
        *val += 1;
    }

    for color in results.keys() {
        predictions.push(Prediction {
            name: color.clone(),
            score: (*results.get(color).unwrap() as f64) / (pixel_count as f64)
        });
    }

    predictions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
}

fn dist(base: u64, supplied: u64) -> u64 {
    return if base > supplied { base - supplied } else { supplied - base }
}
