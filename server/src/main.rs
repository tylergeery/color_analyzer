#![feature(plugin, proc_macro_hygiene, decl_macro, custom_derive)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate image;
extern crate multipart;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde;
extern crate reqwest;
extern crate base64;

mod analyze;
mod colors;

use std::collections::HashMap;
use std::path::Path;
use std::io::{self, Read};
use std::str;
use rocket::Data;
use rocket::response::NamedFile;
use rocket::http::ContentType;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use multipart::server::Multipart;
use multipart::server::save::Entries;
use multipart::server::save::SaveResult::*;


#[derive(FromForm, Deserialize)]
struct URLRequest {
    preference: String,
    url: String
}

struct FormRequest {
    preference: String,
    image: Vec<u8>
}

#[derive(Serialize)]
struct RequestError {
    success: bool,
    error: String
}

fn process_upload(boundary: &str, data: Data) -> io::Result<FormRequest> {
    let mut request = FormRequest {
        preference: String::from(""),
        image: Vec::new()
    };

    // saves all fields, any field longer than 10kB goes to a temporary directory
    // Entries could implement FromData though that would give zero control over
    // how the files are saved; Multipart would be a good impl candidate though
    match Multipart::with_body(data.open(), boundary).save().temp() {
        Full(entries) => process_entries(entries, &mut request),
        Partial(partial, _reason) => {
            process_entries(partial.entries, &mut request)
        },
        Error(e) => return Err(e),
    }

    Ok(request)
}

// having a streaming output would be nice; there's one for returning a `Read` impl
// but not one that you can `write()` to
fn process_entries(entries: Entries, out: &mut FormRequest) {
    let img_key = String::from("image");
    let pref_key = String::from("preference");

    let image = entries.fields.get(&img_key).unwrap();
    let pref = entries.fields.get(&pref_key).unwrap();
    let mut pref_vec: Vec<u8> = Vec::new();

    let mut image_reader = image[0].data.readable().unwrap();
    let mut pref_reader = pref[0].data.readable().unwrap();

    image_reader.read_to_end(&mut out.image).unwrap();
    pref_reader.read_to_end(&mut pref_vec).unwrap();

    out.preference = str::from_utf8(&pref_vec).unwrap().to_string();
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the Image Color Analyzer API!"
}

#[get("/")]
fn upload() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("index.html")).ok()
}

#[post("/", format = "multipart/form-data", data="<data>")]
fn submit(cont_type: &ContentType, data: Data) -> String {
    let (_, boundary) = cont_type.params().find(|&(k, _)| k == "boundary").unwrap();
    let form_request = process_upload(boundary, data).unwrap();

    // analyze Image
    let mut image = image::load_from_memory(&form_request.image).unwrap().to_rgba();
    let mut predictions: Vec<analyze::Prediction> = Vec::new();

    match form_request.preference.as_ref() {
        "center" => {
            image = analyze::center_image(image);
        },
        _ => {}
    }

    let mut color_map: HashMap<String, colors::Color> = HashMap::new();
    colors::parse(&mut color_map);
    analyze::predict(image, color_map, &mut predictions);

    let json = json!(predictions);
    let json_str = json.to_string();

    json_str
}

#[post("/", format = "application/json", data="<request>")]
fn predict(request: Json<URLRequest>) -> String {
    // get file contents from url u
    let url_request = request.into_inner();
    let url = url_request.url;
    let preference = url_request.preference;

    println!("url: {}", &url[..]);

    let mut result = reqwest::get(&url[..]).unwrap();
    let mut buf: Vec<u8> = vec![];
    result.copy_to(&mut buf).unwrap();

    // analyze Image
    let mut image = image::load_from_memory(&buf).unwrap().to_rgba();
    let mut color_map: HashMap<String, colors::Color> = HashMap::new();
    let mut predictions: Vec<analyze::Prediction> = Vec::new();

    match preference.as_ref() {
        "center" => {
            image = analyze::center_image(image);
        },
        _ => {}
    }

    colors::parse(&mut color_map);
    analyze::predict(image, color_map, &mut predictions);

    let json = json!(predictions);
    let json_str = json.to_string();

    json_str
}

#[get("/")]
fn health() -> String {
    String::from("ok")
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/static", StaticFiles::from("static"))
        .mount("/upload", routes![upload])
        .mount("/submit", routes![submit])
        .mount("/predict", routes![predict])
        .mount("/_ah/health", routes![health])
        .launch();
}
