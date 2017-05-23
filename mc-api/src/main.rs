#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
// extern crate serde_json;
// #[macro_use] extern crate serde_json;

use std::io;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use rocket::response::NamedFile;
use rocket_contrib::{Template};
// use rocket::response::content;
// use rocket_contrib::{Template, JSON, Value};
// use rocket::http::hyper;
// use rocket::response::{self, Response, Responder};
// use hyper::header::{Headers, Origin};
// use rocket_contrib::JSON;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open(Path::new("statics/index.html"))
    // let mut context = HashMap::<String, String>::new();
    // context.insert("title", "ssr-with-react-rocket");
    // Template::render("index", &context)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("statics/").join(file)).ok()
    // NamedFile::open(Path::new(&file)).ok()
    // NamedFile::open(Path::new("templates/bundle.js"))
}

// #[derive(Serialize, Deserialize)]
// struct Todo {
//     title: String,
//     done: bool
// }

// #[get("/tasks")]
// fn tasks() -> JSON<Value> {
    // JSON(json!({
    //     "title": "aiueo",
    //     "done": false
    // }))
// }

// #[get("/task/<id>")]
// fn task() -> Template

// #[get("/command")]
// fn command() -> &'static str {
//
// }

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files])
        .mount("/api", routes![index, files])
        .launch();
}
