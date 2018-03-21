#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate serde;
extern crate rocket_contrib;
use rocket_contrib::Json;

#[derive(Serialize)]
struct Task {
  ok: String,
  not_ok: String
}

#[get("/")]
pub fn index() -> Json<Task> {
  Task{ok: "ok", not_ok: "Maybe okay"}
}

fn main() {
  rocket::ignite()
    .mount("/", routes![index])
    .launch();
}
