use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use shipsimulatordb::authenticatie::query;
use shipsimulatordb::authenticatie::models::*;

pub fn check_account(username: &str, password: &str) -> bool {
    let account: Option<User> = query::get_account(username);
    
    match account {
        None => return false,
        Some(user) => return user.encrypted_password == password,
    }
}

pub fn check_cookie_id(username: &str, cookie_id: &str) -> bool {
    let account: Option<User> = query::get_account(username);

    match account {
        None => return false,
        Some(user) => return !user.cookie_id.is_empty() && user.cookie_id == cookie_id,
    }
}

pub fn remove_cookie_id(username: &str) -> bool {
    let empty_cookie_id = String::from("");
    let account: Option<User> = query::get_account(username);
    
    match account {
        None => return false,
        Some(user) => return query::set_cookie_id(&user.username, &empty_cookie_id),
    }
}

pub fn generate_cookie_id(username: &str) -> String {
    let random_id: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .collect();

    query::set_cookie_id(username, &random_id);
    random_id
}