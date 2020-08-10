use crate::authentication::loginmanager::ApiKey;

#[get("/getclaims")]
fn get_claims(key: ApiKey) -> &'static str  {
    if !key.has_role("Admin") {
        return get_claims_no_authenticatie()
    }

    "Welcome to the admin page"
}

#[get("/getclaims", rank = 2)]
fn get_claims_no_authenticatie() -> &'static str  {
    "Welcome to the admin page"
}


#[get("/")]
fn welcome() -> &'static str  {
    "Welcome to the admin page"
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/admin" , routes![get_claims, get_claims_no_authenticatie, welcome])
}