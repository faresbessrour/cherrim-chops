#![feature(plugin)]
#![plugin(rocket_codegen)]
#![
extern crate rocket;
mod index {

  
  #[get("/")]
  pub fn index() -> &'static str {
    "Rocket-Chop Page"
  }

  #[derive(FromForm)]
  struct Traffic {
    card: String,
    state: i8
  }
  #[post("/traffic", data = "<traffic>")]
  pub fn traffic(traffic: From<Traffic>) -> String {
    data.card
  }
}

fn main() {
  rocket::ignite()
    .mount("/", routes![index::index])
    .launch();
}
