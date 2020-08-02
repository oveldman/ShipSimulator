use rocket;
use rocket_contrib::json::Json;

use crate::authentication::User;
use crate::authentication::loginmanager::{self, ApiKey, LoginResult};
use shipsimulatorbl::authenticator;


#[get("/succeed")]
fn succeed(_key: ApiKey) -> &'static str {
    "true - You are logged in!"
}

#[get("/succeed", rank = 2)] 
fn succeed_error() -> &'static str {
    "false - You are not logged in!"
}

#[post("/login", format = "json", data = "<user>")]
fn login(user: Json<User>) -> Json<LoginResult> {
    let username = user.username.to_string();
    let password = user.password.to_string();
    let account_exists: bool = authenticator::check_account(&username, &password);
    let mut new_token: String = String::from("");
    let mut error_message: String = String::from("");
    
    if account_exists {
        new_token = loginmanager::create_token(&username);
    } else {
        error_message = String::from("Username and/or password is not correct!");
    }

    Json(LoginResult {
        token: new_token,
        error: error_message
    })
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/auth", routes![login, succeed, succeed_error])
}