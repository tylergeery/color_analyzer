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
use std::io::{self, Write, Read};
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
    url: String
}

#[derive(Serialize)]
struct RequestError {
    success: bool,
    error: String
}

fn process_upload(boundary: &str, data: Data) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();

    // saves all fields, any field longer than 10kB goes to a temporary directory
    // Entries could implement FromData though that would give zero control over
    // how the files are saved; Multipart would be a good impl candidate though
    match Multipart::with_body(data.open(), boundary).save().temp() {
        Full(entries) => process_entries(entries, &mut out),
        Partial(partial, reason) => {
            writeln!(out, "Request partially processed: {:?}", reason)?;
            if let Some(field) = partial.partial {
                writeln!(out, "Stopped on field: {:?}", field.source.headers)?;
            }

            process_entries(partial.entries, &mut out)
        },
        Error(e) => return Err(e),
    }

    Ok(out)
}

// having a streaming output would be nice; there's one for returning a `Read` impl
// but not one that you can `write()` to
fn process_entries(entries: Entries, out: &mut Vec<u8>) {
    let key = String::from("image");
    let image = entries.fields.get(&key).unwrap();
    let mut reader = image[0].data.readable().unwrap();

    reader.read_to_end(out).unwrap();
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
    let contents = process_upload(boundary, data).unwrap();

    // analyze Image
    let image = image::load_from_memory(&contents).unwrap();
    let mut color_map: HashMap<String, colors::Color> = HashMap::new();
    let mut predictions: Vec<analyze::Prediction> = Vec::new();

    colors::parse(&mut color_map);
    analyze::predict(image, color_map, &mut predictions);

    let json = json!(predictions);
    let json_str = json.to_string();

    json_str
}

#[post("/", format = "application/json", data="<request>")]
fn predict(request: Json<URLRequest>) -> String {
    // get file contents from url u
    let url = request.into_inner().url;
    println!("url: {}", &url[..]);

    let mut result = reqwest::get(&url[..]).unwrap();
    let mut buf: Vec<u8> = vec![];
    result.copy_to(&mut buf).unwrap();
    println!("image contents: {:?}", buf);

    // analyze Image
    let image = image::load_from_memory(&buf).unwrap();
    let mut color_map: HashMap<String, colors::Color> = HashMap::new();
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
