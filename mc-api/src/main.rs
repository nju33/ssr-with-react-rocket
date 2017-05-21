#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
// extern crate serde_json;
// #[macro_use] extern crate serde_json;


// use rocket::response::content;
use rocket_contrib::{JSON, Value};
use rocket::http::hyper;
use rocket::response::{self, Response, Responder};
// use hyper::header::{Headers, Origin};
// use rocket_contrib::JSON;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize, Deserialize)]
struct Todo {
    title: String,
    done: bool
}

#[get("/json")]
fn json() -> JSON<Value> {
    JSON(json!({
        "title": "aiueo",
        "done": false
    }))
}

// #[get("/command")]
// fn command() -> &'static str {
//
// }

fn main() {
    rocket::ignite().mount("/", routes![index, json]).launch();
}
