use chrono::{ DateTime, Utc };
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use shipsimulatorcommon::timechanger;
use shipsimulatordb::authenticatie::query::{self, Account};
use shipsimulatordb::authenticatie::models::*;

const LOGIN_EXPIRED_AFTER_DAYS: i64 = 7;

pub struct Session {
    pub login_succeed: bool,
    pub account: Option<Account>
}

pub fn check_account(username: &str, password: &str) -> Session {
    let account: Option<Account> = query::get_account(username);
    
    match account {
        None => return Session {
            login_succeed:false,
            account: None
        },
        Some(session) => return validate_password(password, session)
    }
}

pub fn check_cookie_id(username: &str, cookie_id: &str) -> bool {
    let account: Option<Account> = query::get_account(username);

    match account {
        None => return false,
        Some(session) => return validate_session(&session.user, cookie_id),
    }
}

pub fn remove_cookie_id(username: &str) -> bool {
    let login_expire_date: DateTime<Utc> = Utc::now();

    let empty_cookie_id = String::from("");
    let account: Option<Account> = query::get_account(username);
    
    match account {
        None => return false,
        Some(session) => return query::set_cookie_id(&session.user.username, &empty_cookie_id, &login_expire_date),
    }
}

pub fn generate_cookie_id(username: &str) -> String {
    let mut login_expire_date: DateTime<Utc> = Utc::now();
    login_expire_date = timechanger::add_time(login_expire_date, LOGIN_EXPIRED_AFTER_DAYS);
    let random_id: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .collect();

    query::set_cookie_id(username, &random_id, &login_expire_date);
    random_id
}

fn validate_password(input_password: &str, account: Account) -> Session {
    Session {
        login_succeed: account.user.encrypted_password == input_password,
        account: Some(account),
    }
}

fn validate_session(user: &User, cookie_id: &str) -> bool {
    let current_time: DateTime<Utc> = Utc::now();

    !user.cookie_id.is_empty() && user.cookie_id == cookie_id && user.login_expired_at > current_time
}