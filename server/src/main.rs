#![feature(plugin, proc_macro_hygiene, decl_macro, custom_derive)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate image;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde;
extern crate reqwest;

mod analyze;
mod colors;

use std::collections::HashMap;
use std::path::Path;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

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
fn predict(request: Json<URLRequest>) -> String {
    // get file contents from url u
    let url = request.into_inner().url;
    println!("url: {}", &url[..]);

    let mut result = reqwest::get(&url[..]).unwrap();
    let mut buf: Vec<u8> = vec![];
    result.copy_to(&mut buf).unwrap();

    // analyze Image
    let image = image::load_from_memory(&buf).unwrap();
    let mut color_map: HashMap<String, Vec<u64>> = HashMap::new();
    let mut predictions: Vec<analyze::Prediction> = Vec::new();

    colors::parse(&mut color_map);
    analyze::predict(image, color_map, &mut predictions);

    let json = json!(predictions);
    let json_str = json.to_string();

    json_str
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/static", StaticFiles::from("static"))
        .mount("/upload", routes![upload])
        .mount("/submit", routes![submit])
        .mount("/predict", routes![predict])
        .launch();
}
