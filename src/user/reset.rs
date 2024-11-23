use rocket::{http::Status, response::content::RawHtml, serde::{json::{json, Json, Value}, Deserialize}};
use rocket_db_pools::Connection;

use crate::DB;

use super::get_user_by_email;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResetData<'r> {
    email: &'r str,
    plaintext_password: &'r str,
}

#[post("/auth/password_reset", format = "json", data = "<data>")]
pub async fn auth_password_reset(mut db: Connection<DB>, data: Json<ResetData<'_>>) -> (Status, Value) {
    let user = match get_user_by_email(&mut db, &data.email).await {
        Ok(val) => val,
        Err(_) => return (Status::BadRequest, json!({"error": "User not found"}))
    };

    if user.pending_password.is_some() {
        return (Status::BadRequest, json!({"error": "Password reset already in progress"}));
    }
    
    (Status::Ok, json!({}))
}

#[get("/auth/password_reset/verify/<token>")]
pub async fn auth_password_reset_verify(db: Connection<DB>, token: &str) -> RawHtml<String> {
    RawHtml("Work in progress".to_owned())
}
