use chrono::{ DateTime, Utc };

#[derive(Clone, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub encrypted_password: String,
    pub cookie_id: String,
    pub login_expired_at: DateTime<Utc>
}

#[derive(Clone, Queryable)]
pub struct Claim {
    pub id: i32,
    pub name: String
}

#[derive(Clone, Queryable)]
pub struct ClaimUser {
    pub id: i32,
    pub claim_id: i32,
    pub user_id: i32
}