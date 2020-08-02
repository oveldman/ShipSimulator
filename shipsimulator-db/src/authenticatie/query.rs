use diesel::prelude::*;

use crate::db;
use crate::db::schema::users::dsl::*;
use crate::authenticatie::models::*;

pub fn get_account(name: &str) -> Option<User> {
    let connection = db::establish_connection();

    let user_found = users
                    .filter(username.eq(name))
                    .limit(1)
                    .load::<User>(&connection)
                    .expect("Error loading user ");

    if user_found.len() == 1 {
        match user_found.get(0) {
            None => return None,
            Some(user) => return Some(user.clone()),
        }
    }
    
    None
}

pub fn set_cookie_id(name: &str, new_cookie_id: &str) -> bool {
    let connection = db::establish_connection();

    let account = diesel::update(users.filter(username.eq(name)))
                .set(cookie_id.eq(new_cookie_id))
                .get_result::<User>(&connection)
                .expect("Error updating user");

    true
}