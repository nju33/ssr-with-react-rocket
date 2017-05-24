extern crate rocket_contrib;
use rocket_contrib::{Value, JSON};

#[get("/tasks")]
pub fn index() -> JSON<Value> {
    JSON(json!({
        "tasks": [
            {
                "id": 1,
                "title": "foo",
                "done": false
            },
            {
                "id": 2,
                "title": "bar",
                "done": false
            },
            {
                "id": 3,
                "title": "baz",
                "done": false
            }
        ]
    }))
}

#[get("/task/<id>")]
pub fn show(id: usize) -> JSON<Value> {
    JSON(json!({
        "id": id,
        "title": "foo",
        "done": false
    }))
}
