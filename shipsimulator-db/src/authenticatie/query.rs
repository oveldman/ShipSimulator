use chrono::{ DateTime, Utc };
use diesel::prelude::*;

use crate::db;
use crate::db::schema::users::dsl::*;
use crate::db::schema::claims::dsl::*;
use crate::authenticatie::models::*;

pub struct Account {
    pub user: User,
    pub password_hash: [u8; 128],
    pub claims: Vec<String>
}

pub fn get_account(name_user: &str) -> Option<Account> {
    let connection = db::establish_connection();

    let user_found: Vec<User> = users
                    .filter(username.eq(name_user))
                    .limit(1)
                    .load::<User>(&connection)
                    .expect("Error loading user");

    if user_found.len() == 1 {
        match user_found.get(0) {
            None => return None,
            Some(user) => return create_user_object(user.clone()),
        }
    }
    
    None
}

pub fn set_password_hash(name_user: &str, password_hash: &str) -> bool {
    let connection = db::establish_connection();

    let password_hash = password_hash.trim_end_matches('\u{0}');

    diesel::update(users.filter(username.eq(name_user)))
    .set(
        encrypted_password.eq(password_hash)
    )
    .get_result::<User>(&connection)
    .expect("Error updating user");

    true
}

pub fn set_cookie_id(name_user: &str, new_cookie_id: &str, login_will_expire_at: &DateTime<Utc>) -> bool {
    let connection = db::establish_connection();

    diesel::update(users.filter(username.eq(name_user)))
                .set((
                    cookie_id.eq(new_cookie_id),
                    login_expired_at.eq(login_will_expire_at)
                ))
                .get_result::<User>(&connection)
                .expect("Error updating user");

    true
}

fn create_user_object(user: User) -> Option<Account> {
    let password_hash: [u8; 128] = correct_password(&user);
    let all_claims: Vec<String> = find_claims(&user);

    Some(Account {
        user: user,
        password_hash: password_hash,
        claims: all_claims
    })
}

fn correct_password(user: &User) -> [u8; 128] {
    let mut padded = [0u8; 128];
    user.encrypted_password
        .as_bytes()
        .iter()
        .enumerate()
        .for_each(|(i, val)| {
            padded[i] = val.clone();
        });
        
    padded
}

fn find_claims(user: &User) -> Vec<String> {
    let connection = db::establish_connection();

    let user_claims: Vec<(ClaimUser, Claim)> = ClaimUser::belonging_to(user)
    .inner_join(claims)
    .load(&connection).expect("Error loading claims");

    filter_claim_names(user_claims)
}

fn filter_claim_names(user_claims: Vec<(ClaimUser, Claim)>) -> Vec<String> {
    let mut all_claims: Vec<String> = Vec::new();

    for claim in user_claims.iter() {
        all_claims.push(claim.1.name.to_string());
    }

    all_claims
}