use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::{prelude::FromRow, query, query_as, Row, SqliteConnection};

pub mod jwt;
pub mod login;
pub mod register;
pub mod reset;

// ██████╗  █████╗ ████████╗ █████╗ ██████╗  █████╗ ███████╗███████╗    ███████╗██╗   ██╗███╗   ██╗ ██████╗████████╗██╗ ██████╗ ███╗   ██╗███████╗
// ██╔══██╗██╔══██╗╚══██╔══╝██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔════╝    ██╔════╝██║   ██║████╗  ██║██╔════╝╚══██╔══╝██║██╔═══██╗████╗  ██║██╔════╝
// ██║  ██║███████║   ██║   ███████║██████╔╝███████║███████╗█████╗      █████╗  ██║   ██║██╔██╗ ██║██║        ██║   ██║██║   ██║██╔██╗ ██║███████╗
// ██║  ██║██╔══██║   ██║   ██╔══██║██╔══██╗██╔══██║╚════██║██╔══╝      ██╔══╝  ██║   ██║██║╚██╗██║██║        ██║   ██║██║   ██║██║╚██╗██║╚════██║
// ██████╔╝██║  ██║   ██║   ██║  ██║██████╔╝██║  ██║███████║███████╗    ██║     ╚██████╔╝██║ ╚████║╚██████╗   ██║   ██║╚██████╔╝██║ ╚████║███████║
// ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚═════╝ ╚═╝  ╚═╝╚══════╝╚══════╝    ╚═╝      ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝   ╚═╝   ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚══════╝

// ██╗   ██╗███████╗███████╗██████╗ 
// ██║   ██║██╔════╝██╔════╝██╔══██╗
// ██║   ██║███████╗█████╗  ██████╔╝
// ██║   ██║╚════██║██╔══╝  ██╔══██╗
// ╚██████╔╝███████║███████╗██║  ██║
//  ╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝

#[derive(Debug, FromRow)]
pub struct UserDB {
    pub user_id: u32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub gender: bool,
    pub verified: bool,
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
    first_name: &str,
    last_name: &str,
    email: &str,
    password: &str,
    gender: char
) -> Result<(), String> {
    match query(
        "INSERT INTO 
                users 
                (user_id, first_name, last_name, email, 
                password, gender, verified) 
                VALUES (?,?,?,?,?,?,0)",
    )
    .bind(id)
    .bind(first_name)
    .bind(last_name)
    .bind(email)
    .bind(password)
    .bind(gender == 'm')
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to insert user into the database: {}", err)),
    }
}

pub async fn get_user_by_id(db: &mut SqliteConnection, user_id: u32) -> Result<UserDB, String> {
    let user: UserDB = match query_as("SELECT * FROM users WHERE user_id = ?")
        .bind(user_id)
        .fetch_optional(db)
        .await
    {
        Ok(row) => match row {
            Some(val) => val,
            None => return Err("User not found".to_owned()),
        },
        Err(err) => return Err(format!("Failed to get user by id: {}", err)),
    };

    Ok(user)
}

pub async fn get_user_by_email(db: &mut SqliteConnection, email: &str) -> Result<UserDB, String> {
    let user: UserDB = match query_as("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(db)
        .await
    {
        Ok(row) => match row {
            Some(val) => val,
            None => return Err("User not found".to_owned()),
        },
        Err(err) => return Err(format!("Failed to get user by email: {}", err)),
    };

    Ok(user)
}

pub async fn update_user_verification_status(
    db: &mut SqliteConnection,
    user_id: u32,
) -> Result<(), String> {
    match query("UPDATE users SET verified = 1 WHERE user_id = ?")
        .bind(user_id)
        .execute(db)
        .await
    {
        Ok(_) => return Ok(()),
        Err(err) => {
            return Err(format!(
                "Failed to update user's verifications status: {}",
                err
            ))
        }
    }
}

// ██╗   ██╗███████╗██████╗ ██╗███████╗██╗ ██████╗ █████╗ ████████╗██╗ ██████╗ ███╗   ██╗
// ██║   ██║██╔════╝██╔══██╗██║██╔════╝██║██╔════╝██╔══██╗╚══██╔══╝██║██╔═══██╗████╗  ██║
// ██║   ██║█████╗  ██████╔╝██║█████╗  ██║██║     ███████║   ██║   ██║██║   ██║██╔██╗ ██║
// ╚██╗ ██╔╝██╔══╝  ██╔══██╗██║██╔══╝  ██║██║     ██╔══██║   ██║   ██║██║   ██║██║╚██╗██║
//  ╚████╔╝ ███████╗██║  ██║██║██║     ██║╚██████╗██║  ██║   ██║   ██║╚██████╔╝██║ ╚████║
//   ╚═══╝  ╚══════╝╚═╝  ╚═╝╚═╝╚═╝     ╚═╝ ╚═════╝╚═╝  ╚═╝   ╚═╝   ╚═╝ ╚═════╝ ╚═╝  ╚═══╝


#[derive(Debug, FromRow)]
pub struct VerificationDB {
    pub user_id: u32,
    pub timestamp: i64,
    pub verification_token: String,
}

pub async fn get_verification_by_id(
    db: &mut SqliteConnection,
    user_id: u32,
) -> Result<VerificationDB, String> {
    let verification: VerificationDB =
        match query_as("SELECT * FROM verifications WHERE user_id = ?")
            .bind(user_id)
            .fetch_optional(db)
            .await
        {
            Ok(row) => match row {
                Some(val) => val,
                None => return Err("Verification not found".to_owned()),
            },
            Err(err) => return Err(format!("Failed to get verification by id: {}", err)),
        };

    Ok(verification)
}

pub async fn get_verification_by_token(
    db: &mut SqliteConnection,
    token: &str,
) -> Result<VerificationDB, String> {
    let verification: VerificationDB =
        match query_as("SELECT * FROM verifications WHERE verification_token = ?")
            .bind(token)
            .fetch_optional(db)
            .await
        {
            Ok(row) => match row {
                Some(val) => val,
                None => return Err("Verification not found".to_owned()),
            },
            Err(err) => return Err(format!("Failed to get verification by id: {}", err)),
        };

    Ok(verification)
}

pub async fn add_verification(
    db: &mut SqliteConnection,
    user_id: u32,
    token: &str,
) -> Result<(), String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs();

    match query(
        "INSERT INTO 
                verifications
                (user_id, timestamp, verification_token) 
                VALUES (?,?,?)",
    )
    .bind(user_id)
    .bind(timestamp as i64)
    .bind(token)
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!(
            "Failed to insert verification into the database: {}",
            err
        )),
    }
}

pub async fn remove_verification(db: &mut SqliteConnection, user_id: u32) -> Result<(), String> {
    match query("DELETE FROM verifications WHERE user_id = ?")
        .bind(user_id)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!(
            "Failed to delete verification from the database: {}",
            err
        )),
    }
}

// ███████╗███████╗███████╗███████╗██╗ ██████╗ ███╗   ██╗
// ██╔════╝██╔════╝██╔════╝██╔════╝██║██╔═══██╗████╗  ██║
// ███████╗█████╗  ███████╗███████╗██║██║   ██║██╔██╗ ██║
// ╚════██║██╔══╝  ╚════██║╚════██║██║██║   ██║██║╚██╗██║
// ███████║███████╗███████║███████║██║╚██████╔╝██║ ╚████║
// ╚══════╝╚══════╝╚══════╝╚══════╝╚═╝ ╚═════╝ ╚═╝  ╚═══╝

#[derive(Debug, FromRow)]
pub struct SessionDB {
    pub user_id: u32,
    pub session_token: String,
    pub timestamp: i64,
    pub valid_until: i64,
}

pub async fn start_session(
    db: &mut SqliteConnection,
    user_id: u32,
    token: &str,
) -> Result<(), String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs();

    match query(
        "INSERT INTO 
                sessions
                (user_id, session_token, timestamp, valid_until) 
                VALUES (?,?,?,?)",
    )
    .bind(user_id)
    .bind(token)
    .bind(timestamp as i64)
    .bind(timestamp as i64 + 2592000)
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!(
            "Failed to insert verification into the database: {}",
            err
        )),
    }
}

pub async fn stop_all_sessions(
    db: &mut SqliteConnection,
    user_id: u32
) -> Result<(), String> {
    match query("DELETE FROM sessions WHERE user_id = ?")
        .bind(user_id)
        .execute(db)
        .await {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to delete sessions: {}", err))
    }
}

pub async fn get_session_count(db: &mut SqliteConnection, user_id: u32) -> Result<u32, String> {
    let query =
        query("SELECT COUNT(session_token) FROM sessions WHERE user_id = ?")
            .bind(user_id)
            .fetch_optional(db)
            .await;

    let val: u32 = match query {
        Ok(row) => {
            let row = row.unwrap();
            row.get(0)
        }
        Err(err) => return Err(format!("Failed to get session count: {}", err))
    };

    Ok(val)
}

// ██████╗  █████╗ ███████╗███████╗██╗    ██╗ ██████╗ ██████╗ ██████╗     ██████╗ ███████╗███████╗███████╗████████╗
// ██╔══██╗██╔══██╗██╔════╝██╔════╝██║    ██║██╔═══██╗██╔══██╗██╔══██╗    ██╔══██╗██╔════╝██╔════╝██╔════╝╚══██╔══╝
// ██████╔╝███████║███████╗███████╗██║ █╗ ██║██║   ██║██████╔╝██║  ██║    ██████╔╝█████╗  ███████╗█████╗     ██║   
// ██╔═══╝ ██╔══██║╚════██║╚════██║██║███╗██║██║   ██║██╔══██╗██║  ██║    ██╔══██╗██╔══╝  ╚════██║██╔══╝     ██║   
// ██║     ██║  ██║███████║███████║╚███╔███╔╝╚██████╔╝██║  ██║██████╔╝    ██║  ██║███████╗███████║███████╗   ██║   
// ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝ ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═════╝     ╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝   ╚═╝   


