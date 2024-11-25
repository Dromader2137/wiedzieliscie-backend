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
    add_verification, create_user, email_taken, get_user_by_id, get_verification_by_id,
    get_verification_by_token, next_user_id, remove_verification, update_user_verification_status,
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterData<'r> {
    email: &'r str,
    plaintext_password: &'r str,
    first_name: &'r str,
    last_name: &'r str,
    gender: char,
}

async fn send_registration_email(email: &str, verification_token: &str) -> Result<(), String> {
    let resend = Resend::default();

    let from = match env::var("WIEDZIELISCIE_BACKEND_FROM_MAIL") {
        Ok(val) => val,
        Err(_) => return Err("From mail not found".to_owned()),
    };
    let subject = "Confirm your registration to WiedzieLIŚCIE";
    let verification_link = match env::var("WIEDZIELISCIE_BACKEND_URL") {
        Ok(val) => val + "/auth/verify/" + verification_token,
        Err(_) => return Err("Url not found".to_owned()),
    };

    let email = CreateEmailBaseOptions::new(from, [email], subject).with_html(&format!(
        "
            <a href=\"{}\">Click this to verify</a>
            <p>If the link above doesn't work just copy this and paste it into a new browser tab: {}</p>
        ",
        verification_link, verification_link
    ));

    if let Err(err) = resend.emails.send(email).await {
        Err(format!("Failed to send email: {}", err))
    } else {
        Ok(())
    }
}

#[post("/auth/register", format = "json", data = "<data>")]
pub async fn auth_register(
    mut db: Connection<DB>,
    data: Json<RegisterData<'_>>,
) -> (Status, Value) {
    let data = data.into_inner();

    match email_taken(&mut db, data.email).await {
        Ok(check) => {
            if check {
                return (Status::BadRequest, json!({"error": "Email already in use"}));
            }
        }
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    }

    let user_id = match next_user_id(&mut db).await {
        Ok(id) => id,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    let token = Uuid::new_v4().to_string();

    if let Err(err) = create_user(
        &mut db,
        user_id,
        data.first_name,
        data.last_name,
        data.email,
        data.plaintext_password,
        data.gender,
    )
    .await
    {
        return (Status::InternalServerError, json!({"error": err}));
    }

    if let Err(err) = add_verification(&mut db, user_id, &token).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    if let Err(err) = send_registration_email(data.email, &token).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Created, json!({"account_id": user_id}))
}

#[post("/auth/resend_verification/<account_id>")]
pub async fn auth_resend_verification(mut db: Connection<DB>, account_id: u32) -> (Status, Value) {
    let user = match get_user_by_id(&mut db, account_id).await {
        Ok(val) => val,
        Err(err) => return (Status::NotFound, json!({"error": err})),
    };

    let verification = match get_verification_by_id(&mut db, account_id).await {
        Ok(val) => val,
        Err(err) => return (Status::NotFound, json!({"error": err})),
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs() as i64;

    if timestamp - verification.timestamp < 60 {
        return (Status::BadRequest, json!({"error": "Too fast"}));
    }

    if let Err(err) = remove_verification(&mut db, user.user_id).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    let token = Uuid::new_v4().to_string();

    if let Err(err) = add_verification(&mut db, user.user_id, &token).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    if let Err(err) = send_registration_email(&user.email, &token).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

pub fn get_verification_page(title: &str, message: &str) -> String {
    format!(
        "
            <head>
            <meta charset=\"utf-8\" />
            <title>WiedzieLIŚCIE verification</title>
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

#[get("/auth/verify/<token>")]
pub async fn auth_verify(mut db: Connection<DB>, token: &str) -> RawHtml<String> {
    let verification = match get_verification_by_token(&mut db, token).await {
        Ok(val) => val,
        Err(err) => return RawHtml(get_verification_page(&"Verification failed", &err)),
    };

    let user = match get_user_by_id(&mut db, verification.user_id).await {
        Ok(val) => val,
        Err(err) => return RawHtml(get_verification_page(&"Verification failed", &err)),
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs() as i64;

    if verification.verification_token != token || timestamp - verification.timestamp > 3600 {
        match remove_verification(&mut db, user.user_id).await {
            Ok(_) => {
                return RawHtml(get_verification_page(
                    &"Verification failed",
                    &"Token invalid",
                ));
            }
            Err(err) => {
                return RawHtml(get_verification_page(&"Verification failed", &err));
            }
        }
    }

    if let Err(err) = update_user_verification_status(&mut db, user.user_id).await {
        return RawHtml(get_verification_page(&"Verification failed", &err));
    }

    RawHtml(get_verification_page(
        &"Verification successful",
        &"You can now close this page and return to the app",
    ))
}

#[cfg(test)]
mod register_tests {
    use std::env;

    use rocket::http::{ContentType, Status};
    use rocket::local::asynchronous::Client;
    use rocket::serde::json::json;
    use rocket_db_pools::{Database, Pool};

    use super::get_verification_page;
    use crate::user::get_verification_by_id;
    use crate::{rocket, DB};

    #[rocket::async_test]
    async fn register_with_verification() {
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
        )
    }

    #[rocket::async_test]
    async fn register_without_verification() {
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
                    "first_name": "Grzegorz123",
                    "last_name": "Brzęczyszczykiewicz",
                    "gender": 'm'
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Created);
    }
}
