use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

use shipsimulatorbl::authenticator;

type HmacSha256 = Hmac<Sha256>;

pub struct ApiKey(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    pub token: String,
    pub error: String
}

pub fn read_token(token: &str) -> BTreeMap<String, String>  {
    let key: Hmac<Sha256> = HmacSha256::new_varkey(b"some-secret").unwrap();
    let claims: BTreeMap<String, String> = token.verify_with_key(&key).unwrap();
    claims
}

pub fn create_token(user_id: &str) -> String {
    let cookie_id: String = authenticator::generate_cookie_id(user_id);

    let key: Hmac<Sha256> = HmacSha256::new_varkey(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("id", cookie_id);
    claims.insert("user", user_id.to_string());
    claims.sign_with_key(&key).unwrap()
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }

        let claims: BTreeMap<String, String> = read_token(keys[0]);
        check_claim(claims)
    }
}

fn check_claim(claims: BTreeMap<String, String>) -> request::Outcome<ApiKey, ()> {
    if claims.len() != 0 && claims.contains_key("user") && claims.contains_key("id") {
        let user: String = String::from(&claims["user"]);
        let cookie_id: String = String::from(&claims["id"]);
        let cookie_found: bool = authenticator::check_cookie_id(&user, &cookie_id);

        if cookie_found {
            return Outcome::Success(ApiKey(user));
        }
    }

    Outcome::Forward(())
}

