pub mod loginmanager;
pub mod web;

use std::fmt::{self, Debug};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClaimTypes {
    Admin,
    Customer,
    Developer,
    Moderator
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditUser {
    pub old_password: String,
    pub new_password: String,
    pub confirm_password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebResult {
    pub succeed: bool,
    pub message: String,
}

impl fmt::Display for ClaimTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}