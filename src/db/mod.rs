use std::env;

use sqlx::{pool::PoolConnection, query, Sqlite, SqliteConnection};

async fn create_user_table(db: &mut SqliteConnection) -> Result<(), String> {
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
        gender bool,
        verified bool,
        last_verification int,
        verification_tokrn varchar(255),
        password_version int,
        pending_password varchar(255),
        last_password_change int,
        password_change_token varchar(255)
    )")
        .execute(db).await {
        Err(err) => {
            if &format!("{}", err) == "error returned from database: (code: 1) table users already exists" {
                return Ok(())
            } else {
                return Err(format!("Failed to create users table: {}", err))
            }
        },
        _ => Ok(())
    }
    
}

async fn create_verification_table(db: &mut SqliteConnection) -> Result<(), String> {
    if let Ok(var) = env::var("WIEDZIELISCIE_BACKEND_RESET_DB") {
        if var.to_lowercase() == "true" || var == "1" {
            query("DROP TABLE verifications").execute(&mut *db).await.ok();
        }
    }

    match query("CREATE TABLE verifications (
        user_id int,
        timestamp int,
        verification_token varchar(255)
    )")
        .execute(db).await {
        Err(err) => {
            if &format!("{}", err) == "error returned from database: (code: 1) table verifications already exists" {
                Ok(())
            } else {
                Err(format!("Failed to create verifications table: {}", err))
            }
        },
        _ => Ok(())
    }
}

async fn create_session_table(db: &mut SqliteConnection) -> Result<(), String> {
    if let Ok(var) = env::var("WIEDZIELISCIE_BACKEND_RESET_DB") {
        if var.to_lowercase() == "true" || var == "1" {
            query("DROP TABLE sessions").execute(&mut *db).await.ok();
        }
    }

    match query("CREATE TABLE sessions (
        user_id int,
        session_token varchar(255),
        timestamp int,
        valid_until int
    )")
        .execute(db).await {
        Err(err) => {
            if &format!("{}", err) == "error returned from database: (code: 1) table sessions already exists" {
                Ok(())
            } else {
                Err(format!("Failed to create sessions table: {}", err))
            }
        },
        _ => Ok(())
    }
}

async fn create_password_reser_table(db: &mut SqliteConnection) -> Result<(), String> {
    if let Ok(var) = env::var("WIEDZIELISCIE_BACKEND_RESET_DB") {
        if var.to_lowercase() == "true" || var == "1" {
            query("DROP TABLE password_resets").execute(&mut *db).await.ok();
        }
    }

    match query("CREATE TABLE password_resets (
        user_id int,
        reset_token varchar(255),
        password varchar(255), 
        timestamp int,
        valid_until int
    )")
        .execute(db).await {
        Err(err) => {
            if &format!("{}", err) == "error returned from database: (code: 1) table password_resets already exists" {
                Ok(())
            } else {
                Err(format!("Failed to create password_resets table: {}", err))
            }
        },
        _ => Ok(())
    }
}

pub async fn create_tables(mut db: PoolConnection<Sqlite>) {
    if let Err(err) = create_user_table(&mut db).await {
        panic!("{}", err);
    }
    
    if let Err(err) = create_verification_table(&mut db).await {
        panic!("{}", err);
    }
    
    if let Err(err) = create_session_table(&mut db).await {
        panic!("{}", err);
    }
    
    if let Err(err) = create_password_reser_table(&mut db).await {
        panic!("{}", err);
    }
}
