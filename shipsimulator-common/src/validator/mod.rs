pub struct ValidateResult {
    pub correct: bool,
    pub error_message: String
}

pub fn password_confirm_compare(new_password: &str, confirm_password: &str) -> ValidateResult {
    let succeed: bool = new_password == confirm_password;
    let mut message: String = String::from("");

    if !succeed {
        message = String::from("The new and confirm password are not equal to eachother!");
    }

    ValidateResult {
        correct: succeed,
        error_message: message
    }
}