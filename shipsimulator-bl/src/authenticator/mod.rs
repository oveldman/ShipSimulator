use shipsimulatordb::authenticatie::query;

pub fn check_account(username: &str, password: &str) -> bool {
    query::get_accounts();
    password == "secret"
}