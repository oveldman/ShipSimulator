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

pub fn generate_cookie_id(username: &str) -> String {
    let random_id: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .collect();

    let update_succeed = query::set_cookie_id(username, &random_id);
    random_id
}