use std::env;

use resend_rs::{types::CreateEmailBaseOptions, Resend};
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

async fn send_password_reset_email(email: &str, reset_token: &str) -> Result<(), String> {
    let resend = Resend::default();

    let from = match env::var("WIEDZIELISCIE_BACKEND_FROM_MAIL") {
        Ok(val) => val,
        Err(_) => return Err("From mail not found".to_owned()),
    };
    let subject = "Confirm your registration to WiedzieLIŚCIE";
    let password_reset_link = match env::var("WIEDZIELISCIE_BACKEND_URL") {
        Ok(val) => val + "/auth/password_reset/verify/" + reset_token,
        Err(_) => return Err("Url not found".to_owned()),
    };

    let email = CreateEmailBaseOptions::new(from, [email], subject).with_html(&format!(
        "
            <a href=\"{}\">Click this to confirm password change</a>
            <p>If the link above doesn't work just copy this and paste it into a new browser tab: {}</p>
        ",
        password_reset_link, password_reset_link
    ));

    if let Err(err) = resend.emails.send(email).await {
        Err(format!("Failed to send email: {}", err))
    } else {
        Ok(())
    }
}

#[post("/auth/password_reset", format = "json", data = "<data>")]
pub async fn auth_password_reset(mut db: Connection<DB>, data: Json<ResetData<'_>>) -> (Status, Value) {
    let user = match get_user_by_email(&mut db, &data.email).await {
        Ok(val) => val,
        Err(_) => return (Status::BadRequest, json!({"error": "User not found"}))
    };

    

    (Status::Ok, json!({}))
}

#[get("/auth/password_reset/verify/<token>")]
pub async fn auth_password_reset_verify(db: Connection<DB>, token: &str) -> RawHtml<String> {
    RawHtml("Work in progress".to_owned())
}
