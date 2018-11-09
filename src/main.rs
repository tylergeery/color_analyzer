#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::path::Path;
use rocket::response::NamedFile;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn upload() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("index.html")).ok()
}

#[post("/"), format="multipart/form-data"]
fn submit() -> &'static str {
    "TODO"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/upload", routes![upload])
        .mount("/submit", routes![submit])
        .launch();
}
