use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

use shipsimulatorbl::authenticator::{self, Session};

const USERNAME_CLAIM: &str = "user";
const ID_CLAIM: &str = "id";
const NAME_CLAIM: &str = "claim";

// This is temporarily. Soon it will go to settings.
const SECRET_KEY: [u8; 30] = *b"eacukoj5aPCnNMruQHsF4amkbNaGgh";

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug)]
pub struct ApiKey {
    pub username: String,
    pub claims: Vec<String>
}

impl ApiKey {
    pub fn new(new_username: String, new_claims: Vec<String>) -> ApiKey {
        ApiKey {
            username: new_username,
            claims: new_claims
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    pub token: String,
    pub error: String
}

pub fn create_token(user_id: &str, session: Session) -> String {
    let cookie_id: String = authenticator::generate_cookie_id(user_id);

    let key: Hmac<Sha256> = HmacSha256::new_varkey(&SECRET_KEY).unwrap();
    let mut claims: BTreeMap<String, String> = BTreeMap::new();
    claims.insert(ID_CLAIM.to_string(), cookie_id.to_string());
    claims.insert(USERNAME_CLAIM.to_string(), user_id.to_string());
    
    match session.account {
        Some(user) => claims = add_claims(claims, user.claims),
        None => (),
    };
    
    claims.sign_with_key(&key).unwrap()
}

pub fn delete_token(key: ApiKey) {
    let username: String = key.username;
    authenticator::remove_cookie_id(&username);
}

pub fn read_token(token: &str) -> BTreeMap<String, String>  {
    let key: Hmac<Sha256> = HmacSha256::new_varkey(&SECRET_KEY).unwrap();
    let claims: BTreeMap<String, String> = token.verify_with_key(&key).unwrap();
    claims
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

fn add_claims(mut session_claims: BTreeMap<String, String>, db_claims :Vec<String>) -> BTreeMap<String, String> {
    let mut claim_id = 0;

    for db_claim_name in db_claims.iter() {
        let claim_name = String::from(NAME_CLAIM) + &claim_id.to_string();
        session_claims.insert(claim_name, db_claim_name.to_string());
        claim_id = claim_id + 1;
    };

    session_claims
} 

fn check_claim(claims: BTreeMap<String, String>) -> request::Outcome<ApiKey, ()> {
    if claims.len() != 0 && claims.contains_key(USERNAME_CLAIM) && claims.contains_key(ID_CLAIM) {
        let user: String = String::from(&claims[USERNAME_CLAIM]);
        let cookie_id: String = String::from(&claims[ID_CLAIM]);
        let cookie_found: bool = authenticator::check_cookie_id(&user, &cookie_id);

        let mut claim_id = 0;
        let mut claim_names: Vec<String> = Vec::new();
        while claim_id < claims.len() {
            let claim_name = String::from(NAME_CLAIM) + &claim_id.to_string();
            if claims.contains_key(&claim_name) {
                claim_names.push(String::from(&claims[&claim_name]));
                claim_id = claim_id + 1;
            } else {
                break;
            }
        }

        if cookie_found {
            return Outcome::Success(ApiKey::new(user, claim_names));
        }
    }

    Outcome::Forward(())
}

