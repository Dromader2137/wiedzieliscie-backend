use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use rocket::serde::{Deserialize, Serialize};
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    uid: u32,
    exp: u64,
}

fn get_secret() -> Option<String> {
    env::var("WIEDZIELISCIE_BACKEND_SECRET").ok()
}

pub fn get_token(user_id: u32) -> Option<String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let jwt_secret = match get_secret() {
        Some(val) => val,
        None => return None,
    };
    let expiration = timestamp + 1209600;

    let claims = Claims {
        uid: user_id,
        exp: expiration,
    };
    let header = Header::new(Algorithm::HS256);
    match encode(
        &header,
        &claims,
        &EncodingKey::from_base64_secret(&jwt_secret).expect("Unable to decode secret"),
    ) {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

pub fn verify_token(token: String) -> Result<TokenData<Claims>, String> {
    let jwt_secret = match get_secret() {
        Some(val) => val,
        None => return Err("Unable to get the secret".to_owned()),
    };
    match decode::<Claims>(
        &token,
        &DecodingKey::from_base64_secret(&jwt_secret).expect("Unable to decode secret"),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(val) => Ok(val),
        Err(err) => Err(err.to_string())
    }
}
