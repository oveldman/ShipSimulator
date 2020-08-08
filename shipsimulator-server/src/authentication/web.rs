use rocket;
use rocket_contrib::json::Json;

use crate::authentication::{EditUser, User, WebResult};
use crate::authentication::loginmanager::{self, ApiKey, LoginResult};
use shipsimulatorbl::authenticator::{self, Session};
use shipsimulatorcommon::validator::{self, ValidateResult};

#[post("/login", format = "json", data = "<user>")]
fn login(user: Json<User>) -> Json<LoginResult> {
    let username = user.username.to_string();
    let password = user.password.to_string();
    let session: Session = authenticator::check_account(&username, &password);
    let mut new_token: String = String::from("");
    let mut error_message: String = String::from("");
    
    if session.login_succeed {
        new_token = loginmanager::create_token(&username, session);
    } else {
        error_message = String::from("Username and/or password is not correct!");
    }

    Json(LoginResult {
        token: new_token,
        error: error_message
    })
}

#[post("/changepassword", format = "json", data="<edit_user>")]
fn change_password(key: ApiKey, edit_user: Json<EditUser>) -> Json<WebResult> {
    let mut succeed: bool = false;
    let mut return_message: String = String::from("");

    let result: ValidateResult = validator::password_confirm_compare(&edit_user.new_password, &edit_user.confirm_password);

    if result.correct {
        let session: Session = authenticator::check_account(&key.username, &edit_user.old_password);

        if session.login_succeed {
            succeed = authenticator::change_password("oveldman", &edit_user.new_password);
        }
        else {
            return_message = String::from("Username and/or password is not correct!");
        }
    }
    else {
        return_message = result.error_message;
    }

    Json(WebResult {
        succeed: succeed,
        message: return_message
    })
}

#[post("/changepassword", format = "json", data="<_edit_user>", rank = 2)]
fn change_password_error(_edit_user: Json<EditUser>) -> Json<WebResult> {
    Json(WebResult {
        succeed: false,
        message: String::from("You are not logged in!")
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
    "true - You are logged in!"
}

#[get("/succeed", rank = 2)] 
fn succeed_error() -> &'static str {
    "false - You are not logged in!"
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/auth", routes![login, logout, logout_message, change_password, change_password_error, succeed, succeed_error])
}