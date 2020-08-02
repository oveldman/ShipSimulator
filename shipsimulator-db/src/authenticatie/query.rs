use diesel::prelude::*;

use crate::db;
use crate::db::schema::users::dsl::*;
use crate::authenticatie::models::*;

pub fn get_accounts() {
    let connection = db::establish_connection();

    let results = users
                    .limit(5)
                    .load::<User>(&connection)
                    .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for user in results {
        println!("{}", user.username);
        println!("----------\n");
        println!("{}", user.cookie_id);
    }                 
}