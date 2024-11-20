use rocket::serde::{json::Json, Deserialize};
use rocket_db_pools::Connection;
use sqlx::{sqlite::SqliteError, SqlitePool};

use crate::DB;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct RegisterData<'r> {
    email: &'r str,
    plaintext_password: &'r str,
    first_name: &'r str,
    last_name: &'r str,
    gender: char
}

async fn email_taken(db: &SqlitePool) -> Result<bool, SqliteError> {

}

#[post("/auth/register", data = "<data>")]
async fn auth_register(mut db: Connection<DB>, data: Json<RegisterData<'_>>) {

}
