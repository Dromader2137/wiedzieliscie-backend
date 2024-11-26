use rocket::{
    http::Status,
    serde::{
        json::{json, Json, Value},
        Deserialize,
    },
};
use rocket_db_pools::Connection;
use uuid::Uuid;

use crate::DB;

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

#[cfg(test)]
mod login_tests {
    use std::env;

    use rocket::http::{ContentType, Status};
    use rocket::local::asynchronous::Client;
    use rocket::serde::json::json;
    use rocket_db_pools::{Database, Pool};

    use crate::user::get_verification_by_id;
    use crate::user::register::get_verification_page;
    use crate::{rocket, DB};

    #[rocket::async_test]
    async fn login() {
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
        println!("{:?}", response.into_string().await);
        assert_eq!(status, Status::Ok)
    }

    #[rocket::async_test]
    async fn login_overload() {
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

        for _ in 0..33 {
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
            println!("{:?}", response.into_string().await);
            assert_eq!(status, Status::Ok)
        }

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
        println!("{:?}", response.into_string().await);
        assert_eq!(status, Status::BadRequest)
    }

    #[rocket::async_test]
    async fn login_without_verification() {
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

        assert_eq!(response.status(), Status::BadRequest)
    }
}
