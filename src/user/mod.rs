use sqlx::{prelude::FromRow, query_as, query, SqliteConnection};

pub mod register;
pub mod login;
pub mod jwt;

#[derive(Debug, FromRow)]
pub struct UserDB {
    pub user_id: u32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub gender: bool,
    pub verified: bool,
    pub last_verification: i64,
    pub verification_tokrn: String,
    pub password_version: u32,
    pub pending_password: Option<String>,
    pub last_password_change: i64,
    pub password_change_token: Option<String>
}

pub async fn get_user_by_id(db: &mut SqliteConnection, user_id: u32) -> Result<UserDB, String> {
    let user: UserDB = match query_as("SELECT * FROM users WHERE user_id = ?")
        .bind(user_id)
        .fetch_optional(db)
        .await
    {
        Ok(row) => {
            if let Some(val) = row {
                val
            } else {
                return Err("User not found".to_owned())
            }
        },
        Err(err) => return Err(format!("Failed to get user by id: {}", err))
    };

    Ok(user)
}

pub async fn get_user_by_email(db: &mut SqliteConnection, email: &str) -> Result<UserDB, String> {
    let user: UserDB = match query_as("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(db)
        .await
    {
        Ok(row) => {
            if let Some(val) = row {
                val
            } else {
                return Err("User not found".to_owned())
            }
        },
        Err(err) => return Err(format!("Failed to get user by email: {}", err))
    };

    Ok(user)
}

pub async fn get_user_by_auth_token(db: &mut SqliteConnection, token: &str) -> Result<UserDB, String> {
    let user: UserDB = match query_as("SELECT * FROM users WHERE verification_tokrn = ?")
        .bind(token)
        .fetch_optional(db)
        .await
    {
        Ok(row) => {
            if let Some(val) = row {
                val
            } else {
                return Err("User not found".to_owned())
            }
        },
        Err(err) => return Err(format!("Failed to get user by token: {}", err))
    };

    Ok(user)
}

pub async fn update_user_verification_timestamp(db: &mut SqliteConnection, user_id: u32, new_timestamp: i64) -> Result<(), String> {
    match query("UPDATE users SET last_verification = ? WHERE user_id = ?")
        .bind(new_timestamp)
        .bind(user_id)
        .execute(db)
        .await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(format!("Failed to update user's verifications timestamp: {}", err))
    }
}

pub async fn update_user_verification_status(db: &mut SqliteConnection, user_id: u32) -> Result<(), String> {
    match query("UPDATE users SET verified = 1 WHERE user_id = ?")
        .bind(user_id)
        .execute(db)
        .await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(format!("Failed to update user's verifications status: {}", err))
    }
}

pub async fn update_user_password_reset_timestamp(db: &mut SqliteConnection, user_id: u32, new_timestamp: i64) -> Result<(), String> {
    match query("UPDATE users SET last_password_change = ? WHERE user_id = ?")
        .bind(new_timestamp)
        .bind(user_id)
        .execute(db)
        .await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(format!("Failed to update user's password reset timestamp: {}", err))
    }
}

pub async fn update_user_password(db: &mut SqliteConnection, user_id: u32) -> Result<(), String> {
    match query("UPDATE users SET password = pending_password WHERE user_id = ?")
        .bind(user_id)
        .execute(db)
        .await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(format!("Failed to update user's password: {}", err))
    }
}
