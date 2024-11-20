use std::env;

use sqlx::{pool::PoolConnection, query, Sqlite, SqliteConnection};

async fn create_user_table<'a>(db: &mut SqliteConnection) -> Result<(), String> {
    if let Ok(var) = env::var("WIEDZIELISCIE_BACKEND_RESET_DB") {
        if var.to_lowercase() == "true" || var == "1" {
            query("DROP TABLE users").execute(&mut *db).await.ok();
        }
    }
    
    match query("CREATE TABLE users (
        user_id int,
        first_name varchar(255),
        last_name varchar(255),
        email varchar(255),
        password varchar(255),
        gender bool
    )")
        .execute(db).await {
        Err(err) => {
            if format!("{}", err) == "error returned from database: (code: 1) table users already exists".trim().to_owned() {
                Ok(())
            } else {
                Err(format!("Failed to create users table: {}", err))
            }
        },
        _ => Ok(())
    }
}

pub async fn create_tables(mut db: PoolConnection<Sqlite>) {
    match create_user_table(&mut db).await {
        Err(err) => { panic!("{}", err) },
        _ => {}
    }
}
