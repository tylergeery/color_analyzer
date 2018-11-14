#![feature(plugin, proc_macro_hygiene, decl_macro, custom_derive)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

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
    let mut result = reqwest::get(&url[..]).unwrap();
    let text = result.text().unwrap();
    let bytes = text.as_bytes();
    // analyze Image

    let image = image::load_from_memory(bytes).unwrap();
    let pixels = image.to_rgba().pixels();
    // output results
    for pix in pixels {
        println!("Got: {}", pix.color_type());
    }
    "yep"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/upload", routes![upload])
        .mount("/submit", routes![submit])
        .mount("/analyze", routes![analyze])
        .launch();
}
