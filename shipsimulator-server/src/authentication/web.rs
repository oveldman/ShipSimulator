use rocket;
use rocket_contrib::json::Json;

use crate::authentication::{EditUser, User, WebResult};
use crate::authentication::loginmanager::{self, ApiKey, LoginResult};
use shipsimulatorbl::authenticator;

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

#[post("/changepassword", format = "json", data="<edit_user>")]
fn change_password(edit_user: Json<EditUser>) -> Json<WebResult> {
    Json(WebResult {
        succeed: false,
        message: String::from("Not finished yet")
    })
}

#[get("/logout")]
fn logout(key: ApiKey) -> &'static str  {
    loginmanager::delete_token(key);
    logout_message()
}

#[get("/logout", rank = 2)]
fn logout_message() -> &'static str  {
    "You are now logout"
}

#[get("/succeed")]
fn succeed(key: ApiKey) -> &'static str {
    println!("{:?}", key);
    "true - You are logged in!"
}

#[get("/succeed", rank = 2)] 
fn succeed_error() -> &'static str {
    "false - You are not logged in!"
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/auth", routes![login, logout, logout_message, succeed, succeed_error])
}