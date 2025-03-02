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

use super::{
    get_session_by_token, get_user_by_email, get_user_by_id, jwt::verify_token, next_user_id,
    retrieve_user_by_email, retrieve_user_by_id, retrieve_user_by_names,
};

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

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RetrieveUserEmailData<'r> {
    email: &'r str,
}

#[post("/user/retrieve/email", format = "json", data = "<data>")]
pub async fn user_retrieve_email(
    mut db: Connection<DB>,
    data: Json<RetrieveUserEmailData<'_>>,
) -> (Status, Value) {
    match retrieve_user_by_email(&mut db, data.email).await {
        Ok(user) => {
            let gender = if user.gender { "m" } else { "f" };
            (
                Status::Ok,
                json!({
                    "account_id": user.account_id,
                    "email": user.email,
                    "first_name": user.first_name,
                    "last_name": user.last_name,
                    "gender": gender,
                    "points": user.points
                }),
            )
        }
        Err(err) => (Status::NotFound, json!(err)),
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RetrieveUserIdData {
    account_id: u32,
}

#[post("/user/retrieve/id", format = "json", data = "<data>")]
pub async fn user_retrieve_id(
    mut db: Connection<DB>,
    data: Json<RetrieveUserIdData>,
) -> (Status, Value) {
    match retrieve_user_by_id(&mut db, data.account_id).await {
        Ok(user) => {
            let gender = if user.gender { "m" } else { "f" };
            (
                Status::Ok,
                json!({
                    "account_id": user.account_id,
                    "email": user.email,
                    "first_name": user.first_name,
                    "last_name": user.last_name,
                    "gender": gender,
                    "points": user.points
                }),
            )
        }
        Err(_) => (Status::NotFound, json!({})),
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RetrieveUserNamesData<'r> {
    first_name: &'r str,
    last_name: &'r str,
}

#[post("/user/retrieve/name", format = "json", data = "<data>")]
pub async fn user_retrieve_name(
    mut db: Connection<DB>,
    data: Json<RetrieveUserNamesData<'_>>,
) -> (Status, Value) {
    match retrieve_user_by_names(&mut db, data.first_name, data.last_name).await {
        Ok(user) => {
            let gender = if user.gender { "m" } else { "f" };
            (
                Status::Ok,
                json!({
                    "account_id": user.account_id,
                    "email": user.email,
                    "first_name": user.first_name,
                    "last_name": user.last_name,
                    "gender": gender,
                    "points": user.points
                }),
            )
        }
        Err(_) => (Status::NotFound, json!({})),
    }
}

#[get("/user/retrieve/count")]
pub async fn user_retrieve_count(mut db: Connection<DB>) -> (Status, Value) {
    match next_user_id(&mut db).await {
        Ok(user) => (Status::Ok, json!(user - 1)),
        Err(_) => (Status::NotFound, json!({})),
    }
}
