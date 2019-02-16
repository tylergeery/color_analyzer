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

use std::path::Path;
use std::io::{self, Read};
use std::str;
use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver};
use std::thread::spawn;
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

struct FormRequest {
    image: Vec<u8>
}

#[derive(Serialize)]
struct RequestError {
    success: bool,
    error: String
}

fn process_upload(boundary: &str, data: Data) -> io::Result<FormRequest> {
    let mut request = FormRequest {
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

    let image = entries.fields.get(&img_key).unwrap();
    let mut image_reader = image[0].data.readable().unwrap();

    image_reader.read_to_end(&mut out.image).unwrap();
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the Image Color Analyzer API!"
}

#[get("/")]
fn upload() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("index.html")).ok()
}

fn run_predictions(rgba_image: image::RgbaImage) -> Vec<Vec<analyze::Prediction>> {
    let prediction_types = vec!("rgb", "rgb_center", "cluster", "cluster_center");
    let mut results: Vec<Vec<analyze::Prediction>> = Vec::new();
    let centered = analyze::center_image(rgba_image.clone());
    let mut receivers: Vec<Receiver<Vec<analyze::Prediction>>> = Vec::new();

    let color_map = Arc::new(colors::parse());

    // TODO: add concurrency
    for pt in prediction_types {
        let (tx, rx) = channel();
        let rgba_img = rgba_image.clone();
        let cntr_img = centered.clone();
        let map_reference = color_map.clone();
        receivers.push(rx);

        let _hd = spawn(move || {
            let mut predictions: Vec<analyze::Prediction> = Vec::new();

            match pt.as_ref() {
                "rgb_center" => {
                    analyze::predict(cntr_img, &map_reference, &mut predictions)
                },
                "cluster" => {
                    analyze::predict_cluster(rgba_img, &map_reference, &mut predictions);
                },
                "cluster_center" => {
                    analyze::predict_cluster(cntr_img, &map_reference, &mut predictions)
                },
                _ => {
                    analyze::predict(rgba_img, &map_reference, &mut predictions);
                },
            };

            tx.send(predictions).unwrap();
        });
    }

    for recv in &receivers {
        results.push(recv.recv().unwrap());
    }

    results
}

#[post("/", format = "multipart/form-data", data="<data>")]
fn submit(cont_type: &ContentType, data: Data) -> String {
    let (_, boundary) = cont_type.params().find(|&(k, _)| k == "boundary").unwrap();
    let form_request = process_upload(boundary, data).unwrap();

    // analyze Image
    let image = image::load_from_memory(&form_request.image).unwrap().to_rgba();
    let predictions = run_predictions(image);

    let json = json!(predictions);
    let json_str = json.to_string();

    json_str
}

#[post("/", format = "application/json", data="<request>")]
fn predict(request: Json<URLRequest>) -> String {
    // get file contents from url u
    let url_request = request.into_inner();
    let url = url_request.url;

    println!("url: {}", &url[..]);

    let mut result = reqwest::get(&url[..]).unwrap();
    let mut buf: Vec<u8> = vec![];
    result.copy_to(&mut buf).unwrap();

    // analyze Image
    let image = image::load_from_memory(&buf).unwrap().to_rgba();
    let predictions = run_predictions(image);

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
