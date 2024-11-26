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
    email_update_in_progress, get_email_update_by_token, get_email_update_by_user_id,
    get_session_by_token, get_user_by_id, jwt::verify_token, remove_email_updates_by_user_id,
    start_email_update, stop_all_sessions, update_user_email,
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResetData<'r> {
    jwt: &'r str,
    new_value: &'r str,
    account_id: u32,
}

async fn send_email_update_email(email: &str, change_token: &str) -> Result<(), String> {
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
    let claims = match verify_token(data.jwt) {
        Ok(val) => val.claims,
        Err(_) => return (Status::BadRequest, json!({"error": "invalid token"})),
    };

    let user_id = claims.uid;
    let session_token = claims.token;

    let user = match get_user_by_id(&mut db, user_id).await {
        Ok(val) => val,
        Err(err) => return (Status::BadRequest, json!({"error": err})),
    };

    if user_id != data.account_id && !user.admin {
        return (
            Status::BadRequest,
            json!({"error": "account_id and token are different"}),
        );
    }

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

#[cfg(test)]
mod update_email_tests {
    use std::env;

    use rocket::http::{ContentType, Status};
    use rocket::local::asynchronous::Client;
    use rocket::serde::{json::json, Deserialize};
    use rocket_db_pools::{Database, Pool};

    use crate::user::register::get_verification_page;
    use crate::user::update_email::get_email_update_page;
    use crate::user::{get_email_update_by_user_id, get_user_by_id, get_verification_by_id};
    use crate::{rocket, DB};

    #[derive(Deserialize)]
    #[serde(crate = "rocket::serde")]
    struct JwtResp {
        jwt: String,
    }

    #[rocket::async_test]
    async fn modify() {
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

        let verification = get_verification_by_id(&mut db.get().await.unwrap(), 1)
            .await
            .unwrap();

        let token = verification.verification_token;

        let response = client
            .get(format!("/auth/verify/{}", token))
            .dispatch()
            .await;

        assert_eq!(
            response.into_string().await,
            Some(get_verification_page(
                &"Verification successful",
                &"You can now close this page and return to the app",
            ))
        );

        let response = client
            .post(uri!("/auth/login"))
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "wiedzieliscie.api.test@proton.me",
                    "plaintext_password": "dupa",
                })
                .to_string(),
            )
            .dispatch()
            .await;

        let status = response.status();
        assert_eq!(status, Status::Ok);

        let jwt = response.into_json::<JwtResp>().await.unwrap().jwt;

        let response_email = client
            .post(uri!("/user/modify/email"))
            .header(ContentType::JSON)
            .body(
                json!({
                    "jwt": jwt,
                    "new_value": "wiedzieliscie.dupa.dupa@proton.me",
                    "account_id": 1
                })
                .to_string(),
            )
            .dispatch()
            .await;

        let response_email_2 = client
            .post(uri!("/user/modify/email"))
            .header(ContentType::JSON)
            .body(
                json!({
                    "jwt": jwt,
                    "new_value": "wiedzieliscie.dupa.dupa@proton.me",
                    "account_id": 1
                })
                .to_string(),
            )
            .dispatch()
            .await;

        let response_email_3 = client
            .post(uri!("/user/modify/email"))
            .header(ContentType::JSON)
            .body(
                json!({
                    "jwt": jwt,
                    "new_value": "wiedzieliscie.dupa.dupa@proton.me",
                    "account_id": 2
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(Status::Ok, response_email.status());
        assert_eq!(Status::BadRequest, response_email_2.status());
        assert_eq!(Status::BadRequest, response_email_3.status());

        let email = get_email_update_by_user_id(&mut db.get().await.unwrap(), 1)
            .await
            .unwrap();

        let response_verify = client
            .get(format!("/user/modify/email/verify/{}", email.update_token))
            .dispatch()
            .await;

        assert_eq!(
            response_verify.into_string().await,
            Some(get_email_update_page(
                &"Email reset successful",
                &"You can now close this page and log into the app using your new email",
            ))
        );

        let user = get_user_by_id(&mut db.get().await.unwrap(), 1)
            .await
            .unwrap();

        assert_eq!(&user.email, "wiedzieliscie.dupa.dupa@proton.me");
    }
}
