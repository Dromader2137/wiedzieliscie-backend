use std::{env, time::{Duration, SystemTime, UNIX_EPOCH}};

use resend_rs::{types::CreateEmailBaseOptions, Resend};
use rocket::{
    http::Status,
    serde::{
        json::{json, Json, Value},
        Deserialize,
    },
};
use rocket_db_pools::Connection;
use sqlx::{query, Row, SqliteConnection};
use uuid::Uuid;

use crate::DB;

use super::get_user_by_id;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterData<'r> {
    email: &'r str,
    plaintext_password: &'r str,
    first_name: &'r str,
    last_name: &'r str,
    gender: char,
}

async fn email_taken<'a>(db: &mut SqliteConnection, email: &'a str) -> Result<bool, &'a str> {
    match query("SELECT user_id FROM users WHERE ? = email")
        .bind(email)
        .fetch_optional(db)
        .await
    {
        Ok(val) => match val {
            Some(_) => Ok(true),
            None => Ok(false),
        },
        Err(_) => Err("Failed to perform a database query"),
    }
}

async fn next_user_id<'a>(db: &mut SqliteConnection) -> Result<u32, &'a str> {
    match query("SELECT MAX(user_id) FROM users")
        .fetch_optional(db)
        .await
    {
        Ok(val) => match val {
            Some(row) => match row.try_get::<u32, _>(0) {
                Ok(id) => Ok(id + 1),
                Err(_) => Err("Database error"),
            },
            None => Ok(1),
        },
        Err(_) => Err("Failed to perform a database query"),
    }
}

async fn create_user(
    db: &mut SqliteConnection,
    id: u32,
    data: &RegisterData<'_>,
    token: &str
) -> Result<(), String> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time").as_secs();

    match query("INSERT INTO 
                users 
                (user_id, first_name, last_name, email, password, gender, verified, last_verification, verification_tokrn) 
                VALUES (?,?,?,?,?,?,0,?,?)")
        .bind(id)
        .bind(data.first_name)
        .bind(data.last_name)
        .bind(data.email)
        .bind(data.plaintext_password)
        .bind(data.gender == 'm')
        .bind(timestamp as i64)
        .bind(token)
        .execute(db).await {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to insert user into the database: {}", err))
    }
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
        Err(_) => return Err("Url not found".to_owned())
    };

    let email = CreateEmailBaseOptions::new(from, [email], subject)
        .with_html(&format!(
        "<body>
            <a href=\"{}\">Verify (if link doesnt work copy this: {})</a>
        </body>"
        , verification_link, verification_link));

    if let Err(err) = resend.emails.send(email).await {
        Err(format!("Failed to send email: {}", err))
    } else {
        Ok(())
    }
}

#[post("/auth/register", format = "json", data = "<data>")]
pub async fn auth_register(mut db: Connection<DB>, data: Json<RegisterData<'_>>) -> (Status, Value) {
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

    if let Err(err) = create_user(&mut db, user_id, &data, &token).await {
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
        Err(err) => return (Status::NotFound, json!({"error": err}))
    };

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time").as_secs() as i64;
    
    if timestamp - user.last_verification < 60 {
        return (Status::BadRequest, json!({"error": "Too fast"}));
    }

    if let Err(err) = send_registration_email(&user.email, &user.verification_tokrn).await {
        return (Status::InternalServerError, json!({"error": err}));
    }
    
    (Status::Ok, json!({}))
}

#[cfg(test)]
mod register_tests {
    use rocket::http::ContentType;
    use rocket::local::blocking::Client;
    use rocket::serde::json::json;

    use crate::rocket;

    #[test]
    fn register_normal() {
        let client = Client::tracked(rocket()).expect("Failed to create client");
        let response = client
            .post(uri!("/auth/register"))
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "dromader2137@proton.me",
                    "plaintext_password": "dupa",
                    "first_name": "Grzegorz",
                    "last_name": "Brzęczyszczykiewicz",
                    "gender": 'm'
                })
                .to_string(),
            )
            .dispatch();

        assert_eq!(
            response.into_string(),
            Some(
                json!({
                    "account_id": 1
                })
                .to_string()
            )
        );
    }
}
