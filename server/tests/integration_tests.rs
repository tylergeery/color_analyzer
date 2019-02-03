extern crate color_analyzer;
extern crate image;

use std::collections::HashMap;
use color_analyzer::{predict, parse, center_image, Color, Prediction};
use image::Rgb;

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

#[test]
fn test_predict() {
    // Given
    let mut test_image = image::DynamicImage::new_rgb8(3, 3);
    let test_image_buffer = test_image.as_mut_rgb8().unwrap();
    let mut colors: HashMap<String, Color> = HashMap::new();
    let mut predictions: Vec<Prediction> = Vec::new();

    colors.insert(
        String::from("red"),
        Color { hex: String::from("#FF0000"), rgb: vec![255, 0, 0]}
    );
    colors.insert(
        String::from("green"),
        Color { hex: String::from("#00FF00"), rgb: vec![0, 255, 0]}
    );
    colors.insert(
        String::from("blue"),
        Color { hex: String::from("#0000FF"), rgb: vec![0, 0, 255]}
    );

    // set mostly red pixels
    test_image_buffer.put_pixel(0, 0, Rgb { data: [230, 10, 10] });
    test_image_buffer.put_pixel(1, 0, Rgb { data: [240, 40, 40] });
    test_image_buffer.put_pixel(1, 1, Rgb { data: [230, 0, 0] });
    test_image_buffer.put_pixel(1, 2, Rgb { data: [255, 0, 50] });
    test_image_buffer.put_pixel(2, 1, Rgb { data: [255, 200, 200] });
    test_image_buffer.put_pixel(2, 2, Rgb { data: [230, 0, 0] });

    // set a couple blue
    test_image_buffer.put_pixel(0, 1, Rgb { data: [3, 50, 150] });
    test_image_buffer.put_pixel(0, 2, Rgb { data: [23, 10, 100] });

    // set a single green
    test_image_buffer.put_pixel(2, 0, Rgb { data: [23, 190, 100] });

    let tmp = image::DynamicImage::ImageRgb8(test_image_buffer.clone());

    // When
    predict(tmp.to_rgba(), colors, &mut predictions);

    // Then
    assert!(predictions[0].name == "red");
    assert!(predictions[0].score > 0.5);
    assert!(predictions[0].hex == "#FF0000");
    assert!(predictions[1].name == "blue");
    assert!(predictions[1].score > 0.2 && predictions[1].score < 0.25);
    assert!(predictions[1].hex == "#0000FF");
    assert!(predictions[2].name == "green");
    assert!(predictions[2].score < 0.12);
    assert!(predictions[2].hex == "#00FF00");
}

#[test]
fn test_center_image() {
    // Given
    let mut test_image = image::DynamicImage::new_rgb8(5, 5);
    let test_image_buffer = test_image.as_mut_rgb8().unwrap();

    let tmp = image::DynamicImage::ImageRgb8(test_image_buffer.clone());

    // When
    let result = center_image(tmp.to_rgba());

    // Then
    println!("dim: {:?}", result.dimensions());
    assert!(result.dimensions() == (3, 3));
}
