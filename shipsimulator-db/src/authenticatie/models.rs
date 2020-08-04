use chrono::{ DateTime, Utc };

#[derive(Clone, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub encrypted_password: String,
    pub cookie_id: String,
    pub login_expired_at: DateTime<Utc>
}
