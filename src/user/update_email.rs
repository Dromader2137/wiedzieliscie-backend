use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use resend_rs::{types::CreateEmailBaseOptions, Resend};
use rocket::{
    http::Status,
    response::content::RawHtml,
    serde::{
        json::{json, Json, Value},
        Deserialize,
    },
};
use rocket_db_pools::Connection;
use uuid::Uuid;

use crate::{util::check_authorized_user_or_admin, DB};

use super::{
    email_update_in_progress, get_email_update_by_token, get_email_update_by_user_id,
    get_user_by_id, jwt::verify_token, remove_email_updates_by_user_id, start_email_update,
    stop_all_sessions, update_user_email,
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResetData<'r> {
    jwt: &'r str,
    new_value: &'r str,
    account_id: u32,
}

async fn send_email_update_email(email: &str, change_token: &str) -> Result<(), String> {
    if env::var("WIEDZIELISCIE_BACKEND_KEIN_MAIL").is_ok() {
        return Ok(());
    }

    let resend = Resend::default();

    let from = match env::var("WIEDZIELISCIE_BACKEND_FROM_MAIL") {
        Ok(val) => val,
        Err(_) => return Err("From mail not found".to_owned()),
    };
    let subject = "Confirm email change";
    let password_reset_link = match env::var("WIEDZIELISCIE_BACKEND_URL") {
        Ok(val) => val + "/user/modify/email/verify/" + change_token,
        Err(_) => return Err("Url not found".to_owned()),
    };

    let email = CreateEmailBaseOptions::new(from, [email], subject).with_html(&format!( "
            <a href=\"{}\">Click this to confirm email change</a>
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

#[post("/user/modify/email", format = "json", data = "<data>")]
pub async fn user_modify_email(
    mut db: Connection<DB>,
    data: Json<ResetData<'_>>,
) -> (Status, Value) {
    if let Some(err) = check_authorized_user_or_admin(&mut db, data.jwt, data.account_id).await {
        return err;
    }

    let claims = match verify_token(data.jwt) {
        Ok(val) => val.claims,
        Err(_) => return (Status::BadRequest, json!({"error": "invalid token"})),
    };
    let user_id = claims.uid;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs() as i64;

    match email_update_in_progress(&mut db, user_id).await {
        Ok(val) => {
            if val {
                let update = match get_email_update_by_user_id(&mut db, user_id).await {
                    Ok(update) => update,
                    Err(err) => return (Status::InternalServerError, json!({"error": err})),
                };

                if timestamp > update.valid_until {
                    match remove_email_updates_by_user_id(&mut db, user_id).await {
                        Err(err) => return (Status::InternalServerError, json!({"error": err})),
                        _ => {}
                    }
                } else {
                    return (Status::BadRequest, json!({"error": "update in progress"}));
                }
            }
        }
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    }

    let token = Uuid::new_v4().to_string();

    if let Err(err) = start_email_update(&mut db, user_id, data.new_value, &token).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    if let Err(err) = send_email_update_email(data.new_value, &token).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

pub fn get_email_update_page(title: &str, message: &str) -> String {
    format!(
        "
            <head>
            <meta charset=\"utf-8\" />
            <title>WiedzieLIŚCIE email update</title>
            </head>
            <body style=\"background-color: black; color: white;\">
            <div style=\"display: flex; justify-content: center; align-items: center; text-align: center; min-height: 100vh; flex-direction: column\">
            <h1>{}</h1> 
            <p>{}</p>
            </div>
            </body>
    ",
        title, message
    )
}

#[get("/user/modify/email/verify/<token>")]
pub async fn user_modify_email_verify(mut db: Connection<DB>, token: &str) -> RawHtml<String> {
    let update = match get_email_update_by_token(&mut db, token).await {
        Ok(val) => val,
        Err(err) => return RawHtml(get_email_update_page(&"Email update failed", &err)),
    };

    let user = match get_user_by_id(&mut db, update.user_id).await {
        Ok(val) => val,
        Err(err) => return RawHtml(get_email_update_page(&"Email update failed", &err)),
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs() as i64;

    if timestamp > update.valid_until {
        return RawHtml(get_email_update_page(
            &"Email update failed",
            &"Email update expired",
        ));
    }

    if let Err(err) = stop_all_sessions(&mut db, user.user_id).await {
        RawHtml(get_email_update_page(&"Email update failed", &err));
    }

    if let Err(err) = update_user_email(&mut db, user.user_id, &update.email).await {
        RawHtml(get_email_update_page(&"Email update failed", &err));
    }

    RawHtml(get_email_update_page(
        &"Email reset successful",
        &"You can now close this page and log into the app using your new email",
    ))
}
