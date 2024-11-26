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

use crate::DB;

use super::{
    get_reset_by_token, get_reset_by_user_id, get_user_by_email, get_user_by_id,
    remove_password_reset_by_user_id, reset_in_progress, start_reset, stop_all_sessions,
    update_user_password,
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResetData<'r> {
    email: &'r str,
    plaintext_password: &'r str,
}

async fn send_password_reset_email(email: &str, reset_token: &str) -> Result<(), String> {
    if env::var("WIEDZIELISCIE_BACKEND_KEIN_MAIL").is_ok() {
        return Ok(());
    }

    let resend = Resend::default();

    let from = match env::var("WIEDZIELISCIE_BACKEND_FROM_MAIL") {
        Ok(val) => val,
        Err(_) => return Err("From mail not found".to_owned()),
    };
    let subject = "Confirm your password reset";
    let password_reset_link = match env::var("WIEDZIELISCIE_BACKEND_URL") {
        Ok(val) => val + "/auth/password_reset/verify/" + reset_token,
        Err(_) => return Err("Url not found".to_owned()),
    };

    let email = CreateEmailBaseOptions::new(from, [email], subject).with_html(&format!( "
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
pub async fn auth_password_reset(
    mut db: Connection<DB>,
    data: Json<ResetData<'_>>,
) -> (Status, Value) {
    let user = match get_user_by_email(&mut db, &data.email).await {
        Ok(val) => val,
        Err(_) => return (Status::BadRequest, json!({"error": "User not found"})),
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs() as i64;

    match reset_in_progress(&mut db, user.user_id).await {
        Ok(val) => {
            if val {
                let reset = match get_reset_by_user_id(&mut db, user.user_id).await {
                    Ok(val) => val,
                    Err(err) => return (Status::InternalServerError, json!({"error": err})),
                };

                if timestamp > reset.valid_until {
                    match remove_password_reset_by_user_id(&mut db, user.user_id).await {
                        Err(err) => return (Status::InternalServerError, json!({"error": err})),
                        _ => {}
                    }
                } else {
                    return (Status::BadRequest, json!({"error": "reset in progress"}));
                }
            }
        }
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    }

    let token = Uuid::new_v4().to_string();

    if let Err(err) = start_reset(&mut db, user.user_id, data.plaintext_password, &token).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    if let Err(err) = send_password_reset_email(&user.email, &token).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

pub fn get_password_reset_page(title: &str, message: &str) -> String {
    format!(
        "
            <head>
            <meta charset=\"utf-8\" />
            <title>WiedzieLIŚCIE password reset</title>
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

#[get("/auth/password_reset/verify/<token>")]
pub async fn auth_password_reset_verify(mut db: Connection<DB>, token: &str) -> RawHtml<String> {
    let reset = match get_reset_by_token(&mut db, token).await {
        Ok(val) => val,
        Err(err) => return RawHtml(get_password_reset_page(&"Password reset failed", &err)),
    };

    let user = match get_user_by_id(&mut db, reset.user_id).await {
        Ok(val) => val,
        Err(err) => return RawHtml(get_password_reset_page(&"Password reset failed", &err)),
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs() as i64;

    if timestamp > reset.valid_until {
        return RawHtml(get_password_reset_page(
            &"Password reset failed",
            &"Password reset expired",
        ));
    }

    if let Err(err) = stop_all_sessions(&mut db, user.user_id).await {
        RawHtml(get_password_reset_page(&"Password reset failed", &err));
    }

    if let Err(err) = update_user_password(&mut db, user.user_id, &reset.password).await {
        RawHtml(get_password_reset_page(&"Password reset failed", &err));
    }

    RawHtml(get_password_reset_page(
        &"Password reset successful",
        &"You can now close this page and log into the app using your new password",
    ))
}

#[cfg(test)]
mod reset_tests {
    use std::env;

    use rocket::http::{ContentType, Status};
    use rocket::local::asynchronous::Client;
    use rocket::serde::json::json;
    use rocket_db_pools::{Database, Pool};
    use sqlx::{query, SqliteConnection};

    use crate::user::reset::get_password_reset_page;
    use crate::user::{get_reset_by_user_id, get_user_by_id};
    use crate::{rocket, DB};

    #[rocket::async_test]
    async fn reset() {
        env::set_var("WIEDZIELISCIE_BACKEND_RESET_DB", "1");

        let client = Client::tracked(rocket())
            .await
            .expect("Failed to create client");
        let response = client
            .post(uri!("/auth/register"))
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "wiedzieliscie.api.test@proton.me",
                    "plaintext_password": "dupa",
                    "first_name": "Grzegorz",
                    "last_name": "Brzęczyszczykiewicz",
                    "gender": 'm'
                })
                .to_string(),
            )
            .dispatch()
            .await;

        let rocket = client.rocket();
        let db = DB::fetch(rocket).unwrap();

        let status = response.status();
        println!("{:?}", response.into_string().await);
        assert_eq!(status, Status::Created);

        let response = client
            .post(uri!("/auth/password_reset"))
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "wiedzieliscie.api.test@proton.me",
                    "plaintext_password": "dupanew",
                })
                .to_string(),
            )
            .dispatch()
            .await;

        let response_2 = client
            .post(uri!("/auth/password_reset"))
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "wiedzieliscie.api.test@proton.me",
                    "plaintext_password": "dupanew",
                })
                .to_string(),
            )
            .dispatch()
            .await;

        let status = response.status();
        println!("{:?}", response.into_string().await);
        assert_eq!(status, Status::Ok);
        assert_eq!(response_2.status(), Status::BadRequest);

        let reset = get_reset_by_user_id(&mut db.get().await.unwrap(), 1)
            .await
            .unwrap();

        let response = client
            .get(format!("/auth/password_reset/verify/{}", reset.reset_token))
            .dispatch()
            .await;

        assert_eq!(
            response.into_string().await,
            Some(get_password_reset_page(
                &"Password reset successful",
                &"You can now close this page and log into the app using your new password",
            ))
        );

        let user = get_user_by_id(&mut db.get().await.unwrap(), 1)
            .await
            .unwrap();

        assert_eq!(&user.password, "dupanew");
    }

    async fn set_valid_to_zero(db: &mut SqliteConnection, reset_token: &str) {
        query("UPDATE password_resets SET valid_until = 0 WHERE reset_token = ?")
            .bind(reset_token)
            .execute(db)
            .await
            .unwrap();
    }

    #[rocket::async_test]
    async fn reset_expired() {
        env::set_var("WIEDZIELISCIE_BACKEND_RESET_DB", "1");

        let client = Client::tracked(rocket())
            .await
            .expect("Failed to create client");
        let response = client
            .post(uri!("/auth/register"))
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "wiedzieliscie.api.test@proton.me",
                    "plaintext_password": "dupa",
                    "first_name": "Grzegorz",
                    "last_name": "Brzęczyszczykiewicz",
                    "gender": 'm'
                })
                .to_string(),
            )
            .dispatch()
            .await;

        let rocket = client.rocket();
        let db = DB::fetch(rocket).unwrap();

        let status = response.status();
        println!("{:?}", response.into_string().await);
        assert_eq!(status, Status::Created);

        let response = client
            .post(uri!("/auth/password_reset"))
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "wiedzieliscie.api.test@proton.me",
                    "plaintext_password": "dupanew",
                })
                .to_string(),
            )
            .dispatch()
            .await;

        let status = response.status();
        println!("{:?}", response.into_string().await);
        assert_eq!(status, Status::Ok);

        let reset = get_reset_by_user_id(&mut db.get().await.unwrap(), 1)
            .await
            .unwrap();

        set_valid_to_zero(&mut db.get().await.unwrap(), &reset.reset_token).await;

        let response = client
            .get(format!("/auth/password_reset/verify/{}", reset.reset_token))
            .dispatch()
            .await;

        assert_eq!(
            response.into_string().await,
            Some(get_password_reset_page(
                &"Password reset failed",
                &"Password reset expired",
            ))
        );

        let response_2 = client
            .post(uri!("/auth/password_reset"))
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "wiedzieliscie.api.test@proton.me",
                    "plaintext_password": "dupanew2",
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(response_2.status(), Status::Ok);

        let reset = get_reset_by_user_id(&mut db.get().await.unwrap(), 1)
            .await
            .unwrap();

        let response = client
            .get(format!("/auth/password_reset/verify/{}", reset.reset_token))
            .dispatch()
            .await;

        assert_eq!(
            response.into_string().await,
            Some(get_password_reset_page(
                &"Password reset successful",
                &"You can now close this page and log into the app using your new password",
            ))
        );

        let user = get_user_by_id(&mut db.get().await.unwrap(), 1)
            .await
            .unwrap();

        assert_eq!(&user.password, "dupanew2");
    }
}
