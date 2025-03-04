use std::time::{SystemTime, UNIX_EPOCH};

use rocket::{
    http::Status,
    serde::{
        json::{json, Json, Value},
        Deserialize,
    },
};
use rocket_db_pools::Connection;

use crate::{util::is_paused, DB};

use super::{get_session_by_token, get_user_by_id, jwt::verify_token, stop_session};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LogoutData<'r> {
    jwt: &'r str,
}

#[post("/auth/logout", format = "json", data = "<data>")]
pub async fn auth_logout(mut db: Connection<DB>, data: Json<LogoutData<'_>>) -> (Status, Value) {
    let claims = match verify_token(data.jwt) {
        Ok(val) => val.claims,
        Err(_) => return (Status::BadRequest, json!({"error": "invalid token"})),
    };

    let user_id = claims.uid;
    let session_token = claims.token;

    match get_user_by_id(&mut db, user_id).await {
        Ok(_) => {}
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

    if let Err(err) = stop_session(&mut db, &session_token).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}
