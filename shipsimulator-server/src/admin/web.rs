#[get("/")]
fn welcome() -> &'static str  {
    "Welcome to the admin page"
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/admin" , routes![welcome])
}