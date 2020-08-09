#![feature(proc_macro_hygiene, decl_macro, plugin)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use shipsimulatorserver::admin;
use shipsimulatorserver::authentication;


#[get("/")]
fn index() -> &'static str  {
    "Welcome to my site!"
}

fn main() {
    let mut rocket = rocket::ignite().mount("/", routes![index]);
    rocket = admin::web::mount(rocket);
    rocket = authentication::web::mount(rocket);
    rocket.launch();
}
