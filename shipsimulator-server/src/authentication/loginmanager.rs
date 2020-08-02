use hmac::{Hmac, NewMac};

use jwt::SignWithKey;
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

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
    let key: Hmac<Sha256> = HmacSha256::new_varkey(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", user_id);
    claims.sign_with_key(&key).unwrap()
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        let claims = read_token(keys[0]);

        if claims.len() != 0 && claims.contains_key("sub") {
            let claim = String::from(&claims["sub"]);
            return Outcome::Success(ApiKey(claim));
        }

        Outcome::Forward(())
    }
}