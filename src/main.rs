#![feature(plugin, proc_macro_hygiene, decl_macro, custom_derive)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::path::Path;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;

#[macro_use]
extern crate serde_derive;
extern crate serde;

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
fn analyze(request: Json<URLRequest>) -> Json<URLRequest> {
    // get file contents from url u
    request
    // analyze Image

    // output results
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/upload", routes![upload])
        .mount("/submit", routes![submit])
        .mount("/analyze", routes![analyze])
        .launch();
}
