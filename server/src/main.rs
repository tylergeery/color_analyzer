#![feature(plugin, proc_macro_hygiene, decl_macro, custom_derive)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate image;

use std::path::Path;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate reqwest;

#[derive(FromForm, Deserialize, Serialize)]
struct URLRequest {
    url: String
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the Image Color Analyzer API!"
}

#[get("/")]
fn upload() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("index.html")).ok()
}

#[post("/", format="multipart/form-data")]
fn submit() -> &'static str {
    // parse multi-part form

    // analyze Image

    // output results
    "TODO"
}

#[post("/", format = "application/json", data="<request>")]
fn analyze(request: Json<URLRequest>) -> &'static str {
    // get file contents from url u
    let url = request.into_inner().url;
    println!("url: {}", &url[..]);

    let mut result = reqwest::get(&url[..]).unwrap();
    let mut buf: Vec<u8> = vec![];
    result.copy_to(&mut buf).unwrap();
    //println!("result: {:?}", buf);

    // analyze Image
    let image = image::load_from_memory(&buf).unwrap();

    // Iterate over all pixels in the image.
    // println!("Pixel count: {}", image.to_rgba().pixels().count());
    let mut red: u64 = 0;
    let mut green: u64 = 0;
    let mut blue: u64 = 0;

    for img in image.to_rgba().pixels() {
        //println!("image data: {:?}", img);
        red += u64::from(img.data[0]);
        green += u64::from(img.data[1]);
        blue += u64::from(img.data[2]);

    }

    println!("red = {}, green = {}, blue = {}", red, green, blue);
    if red > green && red > blue {
        "red"
    } else if green > red && green > blue {
        "green"
    } else {
        "blue"
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/upload", routes![upload])
        .mount("/submit", routes![submit])
        .mount("/analyze", routes![analyze])
        .launch();
}
