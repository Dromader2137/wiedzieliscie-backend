use rocket::{
    http::Status,
    serde::{
        json::{json, Json, Value},
        Deserialize,
    },
};
use rocket_db_pools::Connection;
use uuid::Uuid;

use crate::{util::is_paused, DB};

use super::{get_session_count, get_user_by_email, jwt::get_token, start_session};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginData<'r> {
    email: &'r str,
    plaintext_password: &'r str,
}

#[post("/auth/login", format = "json", data = "<data>")]
pub async fn auth_login(mut db: Connection<DB>, data: Json<LoginData<'_>>) -> (Status, Value) {
    let user = match get_user_by_email(&mut db, data.email).await {
        Ok(val) => val,
        Err(err) => return (Status::BadRequest, json!({"error": err})),
    };

    if is_paused(&mut db).await && !user.admin {
        return (Status::Unauthorized, json!({"error": "Game paused and user isn't admin"}))
    }
    
    if !user.verified {
        return (Status::BadRequest, json!({"error": "User not verified"}));
    }

    match get_session_count(&mut db, user.user_id).await {
        Ok(val) => {
            if val > 32 {
                return (
                    Status::BadRequest,
                    json!({"error": "Session limit exceeded"}),
                );
            }
        }
        Err(err) => {
            return (Status::InternalServerError, json!({"error": err}));
        }
    }

    let token = Uuid::new_v4().to_string();

    let jwt = match get_token(user.user_id, &token) {
        Some(val) => val,
        None => {
            return (
                Status::InternalServerError,
                json!({"error": "Failed to get token"}),
            )
        }
    };

    if let Err(err) = start_session(&mut db, user.user_id, &token).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    if data.plaintext_password == user.password {
        (Status::Ok, json!({"jwt": jwt}))
    } else {
        (Status::BadRequest, json!({"error": "Wrong password"}))
    }
}
