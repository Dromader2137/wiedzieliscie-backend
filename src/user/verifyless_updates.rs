use std::time::{SystemTime, UNIX_EPOCH};

use rocket::{
    http::Status,
    serde::{
        json::{json, Json, Value},
        Deserialize,
    },
};
use rocket_db_pools::Connection;

use crate::DB;

use super::{
    get_session_by_token, get_user_by_id, jwt::verify_token, stop_all_sessions,
    update_user_name_or_gender, update_user_password,
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

    if let Err(err) = update_user_password(&mut db, data.account_id, data.new_value).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    if let Err(err) = stop_all_sessions(&mut db, user_id).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

#[cfg(test)]
mod verifyless_modify_tests {
    use std::env;

    use rocket::http::{ContentType, Status};
    use rocket::local::asynchronous::Client;
    use rocket::serde::{json::json, Deserialize};
    use rocket_db_pools::{Database, Pool};

    use crate::user::register::get_verification_page;
    use crate::user::{get_user_by_id, get_verification_by_id};
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

        let response_first_name = client
            .post(uri!("/user/modify/first_name"))
            .header(ContentType::JSON)
            .body(
                json!({
                    "jwt": jwt,
                    "new_value": "Jan",
                    "account_id": 1
                })
                .to_string(),
            )
            .dispatch()
            .await;

        let response_last_name = client
            .post(uri!("/user/modify/last_name"))
            .header(ContentType::JSON)
            .body(
                json!({
                    "jwt": jwt,
                    "new_value": "Bakszot",
                    "account_id": 1
                })
                .to_string(),
            )
            .dispatch()
            .await;

        let response_gender = client
            .post(uri!("/user/modify/gender"))
            .header(ContentType::JSON)
            .body(
                json!({
                    "jwt": jwt,
                    "new_value": "f",
                    "account_id": 1
                })
                .to_string(),
            )
            .dispatch()
            .await;

        let response_password = client
            .post(uri!("/user/modify/password"))
            .header(ContentType::JSON)
            .body(
                json!({
                    "jwt": jwt,
                    "new_value": "dupsko",
                    "account_id": 1
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(Status::Ok, response_first_name.status());
        assert_eq!(Status::Ok, response_last_name.status());
        assert_eq!(Status::Ok, response_gender.status());
        assert_eq!(Status::Ok, response_password.status());

        let user = get_user_by_id(&mut db.get().await.unwrap(), 1)
            .await
            .unwrap();

        assert_eq!(user.gender, false);
        assert_eq!(&user.last_name, "Bakszot");
        assert_eq!(&user.first_name, "Jan");
        assert_eq!(&user.password, "dupsko");
    }
}
