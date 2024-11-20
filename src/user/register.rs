use rocket::{http::Status, serde::{json::{Value, Json, json}, Deserialize}};
use rocket_db_pools::Connection;
use sqlx::{query, Row, SqliteConnection};

use crate::DB;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterData<'r> {
    email: &'r str,
    plaintext_password: &'r str,
    first_name: &'r str,
    last_name: &'r str,
    gender: char
}

async fn email_taken<'a>(db: &mut SqliteConnection, email: &'a str) -> Result<bool, &'a str> {
    match query("SELECT user_id FROM users WHERE ? = email")
        .bind(email)
        .fetch_optional(db).await {
        Ok(val) => {
            match val {
                Some(_) => Ok(true),
                None => Ok(false)
            }

        },
        Err(_) => Err("Failed to perform a database query")
    }
}

async fn next_user_id<'a>(db: &mut SqliteConnection) -> Result<u32, &'a str> {
    match query("SELECT MAX(user_id) FROM users")
        .fetch_optional(db).await {
        Ok(val) => {
            match val {
                Some(row) => {
                    match row.try_get::<u32, _>(0) {
                        Ok(id) => Ok(id + 1),
                        Err(_) => Err("Database error")
                    }
                },
                None => Ok(1)
            }
        },
        Err(_) => Err("Failed to perform a database query")
    }
}

async fn create_user<'a>(db: &mut SqliteConnection, id: u32, data: RegisterData<'_>) -> Result<(), &'a str> {
    match query("INSERT INTO users (user_id, first_name, last_name, email, password, gender) VALUES (?,?,?,?,?,?)")
        .bind(id)
        .bind(data.first_name)
        .bind(data.last_name)
        .bind(data.email)
        .bind(data.plaintext_password)
        .bind(data.gender == 'm')
        .execute(db).await {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to insert user into the database")
    }
}

#[post("/auth/register", format = "json", data = "<data>")]
pub async fn auth_register(db: Connection<DB>, data: Json<RegisterData<'_>>) -> (Status, Value) {
    let mut db = db.into_inner();
    let data = data.into_inner();

    match email_taken(&mut db, data.email).await {
        Ok(check) => {
            if check {
                return (Status::BadRequest, json!({"error": "Email already in use"}));
            }
        },
        Err(err) => return (Status::InternalServerError, json!({"error": err}))
    }

    let user_id = match next_user_id(&mut db).await {
        Ok(id) => id,
        Err(err) => return (Status::InternalServerError, json!({"error": err}))
    };

    match create_user(&mut db, user_id, data).await {
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
        _ => {}
    }
    
    (Status::Created, json!({"account_id": user_id}))
}

#[cfg(test)]
mod register_tests {
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;
    use rocket::serde::json::json;

    use crate::rocket;

    #[test]
    fn register_normal() {
        let client = Client::tracked(rocket()).expect("Failed to create client");
        let response = client
            .post(uri!("/auth/register"))
            .header(ContentType::JSON)
            .body(json!({
                "email": "dupa@proton.me",
                "plaintext_password": "dupa",
                "first_name": "Grzegorz",
                "last_name": "Brzęczyszczykiewicz",
                "gender": 'm'
            }).to_string())
            .dispatch();

        assert_eq!(response.status(), Status::Created);
        assert_eq!(response.into_string(), Some(
            json!({
                "account_id": 1
            }).to_string()
        ));
    }
}
