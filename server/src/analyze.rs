use std::collections::HashMap;
use image::DynamicImage;

// TODO: return json
pub fn predict(image: DynamicImage, colors: HashMap<String, Vec<u64>>) -> String {
    let mut results: HashMap<String, u64> = HashMap::new();
    let mut winner = String::new();

    for pix in image.to_rgba().pixels() {
        let mut curr: f64 = 1000.0;
        let red = u64::from(pix.data[0]);
        let green = u64::from(pix.data[1]);
        let blue = u64::from(pix.data[2]);

        for key in colors.keys() {
            let color = colors.get(key).unwrap();
            let euclidean = ((red - color[0])^2) + ((green - color[1])^2) + ((blue - color[2])^2);
            let value = f64::sqrt(euclidean as f64);

            if value < curr {
                curr = value;
                winner = key.to_string();
            }
        }

        let val = results.entry(winner.to_string()).or_insert(0);
        *val += 1;
    }

    let mut top: u64 = 0;
    for key in results.keys() {
        let val = results.get(key).unwrap();
        if *val > top {
            top = *val;
            winner = key.to_string();
        }
    }

    winner
}
