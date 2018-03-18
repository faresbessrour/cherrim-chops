#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
mod index {
 #[get("/")]
 pub fn index() -> &'static str {
  "Rocket-Chop Page"
 }
}

fn main() {
  rocket::ignite()
    .mount("/", routes![index::index])
    .launch();
}
