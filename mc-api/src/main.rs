#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate rocket_contrib;
mod statics;
mod api;

use std::io;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
// use rocket::response::NamedFile;
// use rocket_contrib::{Template};
// use rocket::response::content;
// use rocket_contrib::{Template, JSON, Value};
// use rocket::http::hyper;
// use rocket::response::{self, Response, Responder};
// use hyper::header::{Headers, Origin};
// use rocket_contrib::JSON;
//
// #[derive(Serialize, Deserialize)]
// struct Todo {
//     title: String,
//     done: bool
// }

// #[get("/task/<id>")]
// fn task() -> Template

fn main() {
    rocket::ignite()
        .mount("/api", routes![
            api::tasks::index,
            api::tasks::show
        ])
        .mount("/", routes![
            statics::files,
            statics::index,
            statics::dirs
        ])
        .launch();
}
