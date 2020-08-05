pub mod loginmanager;
pub mod web;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditUser {
    pub old_password: String,
    pub new_password: String,
    pub confirm_password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebResult {
    pub succeed: bool,
    pub message: String,
}