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

use crate::{
    user::{delete_user_db, get_delete_request_by_token, get_user_by_id},
    DB,
};

use super::{
    deletion_in_progress, get_delete_request_by_user_id, get_user_by_email,
    remove_delete_request_by_user_id, start_delete,
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteData<'r> {
    email: &'r str,
}

async fn send_delete_user_email(email: &str, delete_token: &str) -> Result<(), String> {
    if env::var("WIEDZIELISCIE_BACKEND_KEIN_MAIL").is_ok() {
        return Ok(());
    }

    let resend = Resend::default();

    let from = match env::var("WIEDZIELISCIE_BACKEND_FROM_MAIL") {
        Ok(val) => val,
        Err(_) => return Err("From mail not found".to_owned()),
    };
    let subject = "Confirm your account deletion request";
    let password_reset_link = match env::var("WIEDZIELISCIE_BACKEND_URL") {
        Ok(val) => val + "/auth/delete_user/verify/" + delete_token,
        Err(_) => return Err("Url not found".to_owned()),
    };

    let email = CreateEmailBaseOptions::new(from, [email], subject).with_html(&format!( "
            <a href=\"{}\">Click this to confirm account deletion</a>
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

#[post("/auth/delete_user", format = "json", data = "<data>")]
pub async fn delete_user(mut db: Connection<DB>, data: Json<DeleteData<'_>>) -> (Status, Value) {
    let user = match get_user_by_email(&mut db, data.email).await {
        Ok(val) => val,
        Err(_) => return (Status::BadRequest, json!({"error": "User not found"})),
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs() as i64;

    match deletion_in_progress(&mut db, user.user_id).await {
        Ok(val) => {
            if val {
                let reset = match get_delete_request_by_user_id(&mut db, user.user_id).await {
                    Ok(val) => val,
                    Err(err) => return (Status::InternalServerError, json!({"error": err})),
                };

                if timestamp > reset.valid_until {
                    match remove_delete_request_by_user_id(&mut db, user.user_id).await {
                        Err(err) => return (Status::InternalServerError, json!({"error": err})),
                        _ => {}
                    }
                } else {
                    return (
                        Status::BadRequest,
                        json!({"error": "account deletion in progress"}),
                    );
                }
            }
        }
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    }

    let token = Uuid::new_v4().to_string();

    if let Err(err) = start_delete(&mut db, user.user_id, &token).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    if let Err(err) = send_delete_user_email(&user.email, &token).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

pub fn get_delete_user_page(title: &str, message: &str) -> String {
    format!(
        "
            <head>
            <meta charset=\"utf-8\" />
            <title>WiedzieLIŚCIE account deletion</title>
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

#[get("/auth/delete_user/verify/<token>")]
pub async fn auth_password_reset_verify(mut db: Connection<DB>, token: &str) -> RawHtml<String> {
    println!("DELETE USER");

    let reset = match get_delete_request_by_token(&mut db, token).await {
        Ok(val) => val,
        Err(err) => return RawHtml(get_delete_user_page(&"Password reset failed", &err)),
    };

    let user = match get_user_by_id(&mut db, reset.user_id).await {
        Ok(val) => val,
        Err(err) => return RawHtml(get_delete_user_page(&"Password reset failed", &err)),
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs() as i64;

    if timestamp > reset.valid_until {
        return RawHtml(get_delete_user_page(
            &"Account deletion failed",
            &"Account deletion expired",
        ));
    }

    if let Err(err) = delete_user_db(&mut db, user.user_id).await {
        RawHtml(get_delete_user_page(&"Password reset failed", &err));
    }

    RawHtml(get_delete_user_page(
        &"Account deletion successful",
        &"You can now close this page",
    ))
}
