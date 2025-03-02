use rocket::{
    http::Status,
    serde::{
        json::{json, Json, Value},
        Deserialize,
    },
};
use rocket_db_pools::Connection;

use crate::{util::check_authorized_user_or_admin, DB};

use super::{
    jwt::verify_token, stop_all_sessions, update_user_name_or_gender, update_user_password,
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SimpleModifyData<'r> {
    jwt: &'r str,
    new_value: &'r str,
    account_id: u32,
}

#[post("/user/modify/first_name", format = "json", data = "<data>")]
pub async fn user_modify_first_name(
    mut db: Connection<DB>,
    data: Json<SimpleModifyData<'_>>,
) -> (Status, Value) {
    if let Some(err) = check_authorized_user_or_admin(&mut db, data.jwt, data.account_id).await {
        return err;
    }

    if let Err(err) =
        update_user_name_or_gender(&mut db, data.account_id, "first_name", data.new_value).await
    {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

#[post("/user/modify/last_name", format = "json", data = "<data>")]
pub async fn user_modify_last_name(
    mut db: Connection<DB>,
    data: Json<SimpleModifyData<'_>>,
) -> (Status, Value) {
    if let Some(err) = check_authorized_user_or_admin(&mut db, data.jwt, data.account_id).await {
        return err;
    }

    if let Err(err) =
        update_user_name_or_gender(&mut db, data.account_id, "last_name", data.new_value).await
    {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

#[post("/user/modify/gender", format = "json", data = "<data>")]
pub async fn user_modify_gender(
    mut db: Connection<DB>,
    data: Json<SimpleModifyData<'_>>,
) -> (Status, Value) {
    if data.new_value != "m" && data.new_value != "f" {
        return (Status::BadRequest, json!({"error": "invalid new value"}));
    }

    if let Some(err) = check_authorized_user_or_admin(&mut db, data.jwt, data.account_id).await {
        return err;
    }

    if let Err(err) =
        update_user_name_or_gender(&mut db, data.account_id, "gender", data.new_value).await
    {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

#[post("/user/modify/password", format = "json", data = "<data>")]
pub async fn user_modify_password(
    mut db: Connection<DB>,
    data: Json<SimpleModifyData<'_>>,
) -> (Status, Value) {
    if let Some(err) = check_authorized_user_or_admin(&mut db, data.jwt, data.account_id).await {
        return err;
    }

    let claims = match verify_token(data.jwt) {
        Ok(val) => val.claims,
        Err(_) => return (Status::BadRequest, json!({"error": "invalid token"})),
    };

    let user_id = claims.uid;

    if let Err(err) = update_user_password(&mut db, data.account_id, data.new_value).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    if let Err(err) = stop_all_sessions(&mut db, user_id).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}
