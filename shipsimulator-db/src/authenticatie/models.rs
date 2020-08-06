use chrono::{ DateTime, Utc };

use crate::db::schema::*;

#[derive(Associations, Clone, Debug, Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub encrypted_password: String,
    pub cookie_id: String,
    pub login_expired_at: DateTime<Utc>
}

#[derive(Associations, Clone, Debug, Identifiable, Queryable)]
pub struct Claim {
    pub id: i32,
    pub name: String
}

#[derive(Associations, Clone, Debug, Identifiable, Queryable)]
#[table_name = "claim_user"]
#[belongs_to(User, foreign_key = "user_id")]
pub struct ClaimUser {
    pub id: i32,
    pub claim_id: i32,
    pub user_id: i32
}