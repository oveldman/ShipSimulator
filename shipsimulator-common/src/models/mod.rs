use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnResult {
    pub succeed: bool,
    pub message: String
}

impl ReturnResult {
    pub fn new(succeed: bool, message: String) -> ReturnResult {
        ReturnResult {
            succeed: succeed,
            message: message
        }
    }
}