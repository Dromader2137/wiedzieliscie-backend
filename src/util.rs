use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    user::{get_session_by_token, get_user_by_id, jwt::verify_token},
    DB,
};

use rocket::{
    http::Status,
    serde::json::{json, Value},
};
use rocket_db_pools::Connection;

pub async fn check_authorized_user(
    mut db: &mut Connection<DB>,
    jwt: &str,
) -> Option<(Status, Value)> {
    let claims = match verify_token(jwt) {
        Ok(val) => val.claims,
        Err(_) => return Some((Status::BadRequest, json!({"error": "invalid token"}))),
    };

    let user_id = claims.uid;
    let session_token = claims.token;

    if let Err(err) = get_user_by_id(&mut db, user_id).await {
        return Some((Status::BadRequest, json!({"error": err})));
    };

    let sessions = match get_session_by_token(&mut db, &session_token).await {
        Ok(val) => val,
        Err(err) => return Some((Status::BadRequest, json!({"error": err}))),
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs() as i64;

    if timestamp > sessions.valid_until {
        return Some((Status::BadRequest, json!({"error": "token expired"})));
    }

    None
}

pub async fn check_authorized_admin(
    mut db: &mut Connection<DB>,
    jwt: &str,
) -> Option<(Status, Value)> {
    let claims = match verify_token(jwt) {
        Ok(val) => val.claims,
        Err(_) => return Some((Status::BadRequest, json!({"error": "invalid token"}))),
    };

    let user_id = claims.uid;
    let session_token = claims.token;

    let user = match get_user_by_id(&mut db, user_id).await {
        Ok(val) => val,
        Err(err) => return Some((Status::BadRequest, json!({"error": err}))),
    };

    if !user.admin {
        return Some((Status::BadRequest, json!({"error": "user is not admin"})));
    }

    let sessions = match get_session_by_token(&mut db, &session_token).await {
        Ok(val) => val,
        Err(err) => return Some((Status::BadRequest, json!({"error": err}))),
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs() as i64;

    if timestamp > sessions.valid_until {
        return Some((Status::BadRequest, json!({"error": "token expired"})));
    }

    None
}

pub async fn check_authorized_user_or_admin(
    mut db: &mut Connection<DB>,
    jwt: &str,
    account_id: u32,
) -> Option<(Status, Value)> {
    let claims = match verify_token(jwt) {
        Ok(val) => val.claims,
        Err(_) => return Some((Status::BadRequest, json!({"error": "invalid token"}))),
    };

    let user_id = claims.uid;
    let session_token = claims.token;

    let user = match get_user_by_id(&mut db, user_id).await {
        Ok(val) => val,
        Err(err) => return Some((Status::BadRequest, json!({"error": err}))),
    };

    if user_id != account_id && !user.admin {
        return Some((
            Status::BadRequest,
            json!({"error": "account_id and token are different"}),
        ));
    }

    let sessions = match get_session_by_token(&mut db, &session_token).await {
        Ok(val) => val,
        Err(err) => return Some((Status::BadRequest, json!({"error": err}))),
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs() as i64;

    if timestamp > sessions.valid_until {
        return Some((Status::BadRequest, json!({"error": "token expired"})));
    }

    None
}
