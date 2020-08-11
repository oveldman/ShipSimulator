use chrono::{ DateTime, Utc };
use diesel::prelude::*;

use crate::db;
use crate::db::schema::users::dsl::*;
use crate::db::schema::claims::dsl::*;
use crate::authenticatie::models::*;

pub fn get_claims(user_name: &str) {

}