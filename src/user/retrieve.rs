use std::time::{SystemTime, UNIX_EPOCH};

use rocket::{
    http::Status,
    serde::{
        json::{json, Json, Value},
        Deserialize,
    },
};
use rocket_db_pools::Connection;

use crate::DB;

use super::{get_session_by_token, get_user_by_id, jwt::verify_token};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RetrieveUserData<'r> {
    jwt: &'r str,
}

#[post("/auth/retrieve_user", format = "json", data = "<data>")]
pub async fn auth_retrieve_user(
    mut db: Connection<DB>,
    data: Json<RetrieveUserData<'_>>,
) -> (Status, Value) {
    let claims = match verify_token(data.jwt) {
        Ok(val) => val.claims,
        Err(_) => return (Status::BadRequest, json!({"error": "invalid token"})),
    };

    let user_id = claims.uid;
    let session_token = claims.token;

    let user = match get_user_by_id(&mut db, user_id).await {
        Ok(val) => val,
        Err(_) => return (Status::BadRequest, json!({"error": "user not found"})),
    };

    let sessions = match get_session_by_token(&mut db, &session_token).await {
        Ok(val) => val,
        Err(err) => return (Status::BadRequest, json!({"error": err})),
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs() as i64;

    if timestamp > sessions.valid_until {
        return (Status::BadRequest, json!({"error": "token expired"}));
    }

    let gender = if user.gender { "m" } else { "f" };

    (
        Status::Ok,
        json!({
            "account_id": user.user_id,
            "email": user.email,
            "first_name": user.first_name,
            "last_name": user.last_name,
            "gender": gender
        }),
    )
}
