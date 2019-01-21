extern crate serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Color {
    pub rgb: Vec<u64>,
    pub hex: String
}

pub fn parse(colors: &mut HashMap<String, Color>) {
    let mut file = File::open("/usr/src/app/src/colors.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json: HashMap<String, String> = serde_json::from_str(&data[..]).unwrap();

    for key in json.keys() {
        let s = json.get(key).unwrap().trim_left_matches('#');
        colors.insert(
            key.to_string(),
            Color {
                hex: json.get(key).unwrap().to_string(),
                rgb: vec![
                    u64::from_str_radix(s.get(0..2).unwrap(), 16).unwrap(),
                    u64::from_str_radix(s.get(2..4).unwrap(), 16).unwrap(),
                    u64::from_str_radix(s.get(4..).unwrap(), 16).unwrap()
                ]
            }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut colors: HashMap<String, Color> = HashMap::new();

        parse(&mut colors);

        let red = colors.get("red").unwrap();
        let green = colors.get("green").unwrap();
        let blue = colors.get("blue").unwrap();

        assert!(red.rgb[0] == 255);
        assert!(red.rgb[1] == 0);
        assert!(red.rgb[2] == 0);
        assert!(red.hex == "#FF0000");

        assert!(green.rgb[0] == 0);
        assert!(green.rgb[1] == 255);
        assert!(green.rgb[2] == 0);
        assert!(green.hex == "#00FF00");

        assert!(blue.rgb[0] == 0);
        assert!(blue.rgb[1] == 0);
        assert!(blue.rgb[2] == 255);
        assert!(blue.hex == "#0000FF");
    }
}
