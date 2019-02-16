extern crate serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct Color {
    pub rgb: Vec<u64>,
    pub hex: String
}

pub fn parse() -> HashMap<String, Color> {
    let mut colors: HashMap<String, Color> = HashMap::new();
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

    colors
}
