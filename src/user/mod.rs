use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::{prelude::FromRow, query, query_as, Row, SqliteConnection};

pub mod jwt;
pub mod login;
pub mod logout;
pub mod register;
pub mod reset;
pub mod retrieve;
pub mod update_email;
pub mod verifyless_updates;
pub mod delete_user;

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
    pub admin: bool,
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
    gender: char,
) -> Result<(), String> {
    match query(
        "INSERT INTO 
                users 
                (user_id, first_name, last_name, email, 
                password, gender, verified, admin) 
                VALUES (?,?,?,?,?,?,0,0)",
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
        Ok(_) => Ok(()),
        Err(err) => Err(format!(
            "Failed to update user's verifications status: {}",
            err
        )),
    }
}

pub async fn update_user_password(
    db: &mut SqliteConnection,
    user_id: u32,
    password: &str,
) -> Result<(), String> {
    match query("UPDATE users SET password = ? WHERE user_id = ?")
        .bind(password)
        .bind(user_id)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to update user's password: {}", err)),
    }
}

pub async fn update_user_email(
    db: &mut SqliteConnection,
    user_id: u32,
    email: &str,
) -> Result<(), String> {
    match query("UPDATE users SET email = ? WHERE user_id = ?")
        .bind(email)
        .bind(user_id)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to update user's password: {}", err)),
    }
}

pub async fn delete_user_db(
    db: &mut SqliteConnection,
    user_id: u32
) -> Result<(), String> {
    let user: UserDB = match get_user_by_id(db, user_id).await {
        Ok(val) => val,
        Err(err) => return Err(format!("Failed to get user: {}", err)),
    };
    println!("DELETE CALLED");

    match query(
        "INSERT INTO 
                deleted_users 
                (user_id, first_name, last_name, email, 
                password, gender, verified, admin) 
                VALUES (?,?,?,?,?,?,?,?)",
        )
        .bind(user.user_id)
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.email)
        .bind(user.password)
        .bind(user.gender)
        .bind(user.verified)
        .bind(user.admin)
        .execute(&mut *db)
        .await
        {
            Ok(_) => (),
            Err(err) => return Err(format!("Failed to delete the user: {}", err)),
        }

    match query("DELETE FROM users WHERE user_id = ?")
        .bind(user.user_id)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to delete the user: {}", err)),
    }
}

pub async fn update_user_name_or_gender(
    db: &mut SqliteConnection,
    user_id: u32,
    field: &str,
    data: &str,
) -> Result<(), String> {
    if field == "gender" {
        match query(&format!("UPDATE users SET {} = ? WHERE user_id = ?", field))
            .bind(data == "m")
            .bind(user_id)
            .execute(db)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to update user's data: {}", err)),
        }
    } else if field == "last_name" || field == "first_name" {
        match query(&format!("UPDATE users SET {} = ? WHERE user_id = ?", field))
            .bind(data)
            .bind(user_id)
            .execute(db)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to update user's data: {}", err)),
        }
    } else {
        Err("Invalid field".to_owned())
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

pub async fn stop_all_sessions(db: &mut SqliteConnection, user_id: u32) -> Result<(), String> {
    match query("DELETE FROM sessions WHERE user_id = ?")
        .bind(user_id)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to delete sessions: {}", err)),
    }
}

pub async fn stop_session(db: &mut SqliteConnection, token: &str) -> Result<(), String> {
    match query("DELETE FROM sessions WHERE session_token = ?")
        .bind(token)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to delete session: {}", err)),
    }
}

pub async fn get_session_count(db: &mut SqliteConnection, user_id: u32) -> Result<u32, String> {
    let query = query("SELECT COUNT(session_token) FROM sessions WHERE user_id = ?")
        .bind(user_id)
        .fetch_optional(db)
        .await;

    let val: u32 = match query {
        Ok(row) => {
            let row = row.unwrap();
            row.get(0)
        }
        Err(err) => return Err(format!("Failed to get session count: {}", err)),
    };

    Ok(val)
}

pub async fn get_session_by_token(
    db: &mut SqliteConnection,
    token: &str,
) -> Result<SessionDB, String> {
    let password_reset: SessionDB = match query_as("SELECT * FROM sessions WHERE session_token = ?")
        .bind(token)
        .fetch_optional(db)
        .await
    {
        Ok(row) => match row {
            Some(val) => val,
            None => return Err("Session not found".to_owned()),
        },
        Err(err) => return Err(format!("Failed to get session by token: {}", err)),
    };

    Ok(password_reset)
}

// Delete User

#[derive(Debug, FromRow)]
pub struct AccountDeleteRequestDB {
    pub user_id: u32,
    pub delete_token: String,
    pub timestamp: i64,
    pub valid_until: i64,
}

pub async fn deletion_in_progress(db: &mut SqliteConnection, user_id: u32) -> Result<bool, String> {
    match query("SELECT user_id FROM delete_requests WHERE user_id = ?")
        .bind(user_id)
        .fetch_optional(db)
        .await
    {
        Ok(val) => match val {
            Some(_) => Ok(true),
            None => Ok(false),
        },
        Err(_) => Err("Failed to perform a database query".to_owned()),
    }
}

pub async fn start_delete(
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
                delete_requests
                (user_id, delete_token, timestamp, valid_until) 
                VALUES (?,?,?,?,?)",
    )
    .bind(user_id)
    .bind(token)
    .bind(timestamp as i64)
    .bind(timestamp as i64 + 300)
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!(
            "Failed to insert delete request into the database: {}",
            err
        )),
    }
}

pub async fn get_delete_request_by_token(
    db: &mut SqliteConnection,
    token: &str,
) -> Result<AccountDeleteRequestDB, String> {
    let delete_request: AccountDeleteRequestDB =
        match query_as("SELECT * FROM delete_requests WHERE delete_token = ?")
            .bind(token)
            .fetch_optional(db)
            .await
        {
            Ok(row) => match row {
                Some(val) => val,
                None => return Err("Delete request not found".to_owned()),
            },
            Err(err) => return Err(format!("Failed to get delete request by token: {}", err)),
        };

    Ok(delete_request)
}

pub async fn get_delete_request_by_user_id(
    db: &mut SqliteConnection,
    user_id: u32,
) -> Result<AccountDeleteRequestDB, String> {
    let delete_request: AccountDeleteRequestDB =
        match query_as("SELECT * FROM delete_requests WHERE user_id = ?")
            .bind(user_id)
            .fetch_optional(db)
            .await
        {
            Ok(row) => match row {
                Some(val) => val,
                None => return Err("Delete request not found".to_owned()),
            },
            Err(err) => return Err(format!("Failed to get delete request by token: {}", err)),
        };

    Ok(delete_request)
}

pub async fn remove_delete_request_by_user_id(
    db: &mut SqliteConnection,
    user_id: u32,
) -> Result<(), String> {
    match query("DELETE FROM delete_requests user_id = ?")
        .bind(user_id)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to delete delete_request: {}", err)),
    }
}

// ██████╗  █████╗ ███████╗███████╗██╗    ██╗ ██████╗ ██████╗ ██████╗     ██████╗ ███████╗███████╗███████╗████████╗
// ██╔══██╗██╔══██╗██╔════╝██╔════╝██║    ██║██╔═══██╗██╔══██╗██╔══██╗    ██╔══██╗██╔════╝██╔════╝██╔════╝╚══██╔══╝
// ██████╔╝███████║███████╗███████╗██║ █╗ ██║██║   ██║██████╔╝██║  ██║    ██████╔╝█████╗  ███████╗█████╗     ██║
// ██╔═══╝ ██╔══██║╚════██║╚════██║██║███╗██║██║   ██║██╔══██╗██║  ██║    ██╔══██╗██╔══╝  ╚════██║██╔══╝     ██║
// ██║     ██║  ██║███████║███████║╚███╔███╔╝╚██████╔╝██║  ██║██████╔╝    ██║  ██║███████╗███████║███████╗   ██║
// ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝ ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═════╝     ╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝   ╚═╝

#[derive(Debug, FromRow)]
pub struct PasswordResetDB {
    pub user_id: u32,
    pub reset_token: String,
    pub password: String,
    pub timestamp: i64,
    pub valid_until: i64,
}

pub async fn reset_in_progress(db: &mut SqliteConnection, user_id: u32) -> Result<bool, String> {
    match query("SELECT user_id FROM password_resets WHERE user_id = ?")
        .bind(user_id)
        .fetch_optional(db)
        .await
    {
        Ok(val) => match val {
            Some(_) => Ok(true),
            None => Ok(false),
        },
        Err(_) => Err("Failed to perform a database query".to_owned()),
    }
}

pub async fn start_reset(
    db: &mut SqliteConnection,
    user_id: u32,
    password: &str,
    token: &str,
) -> Result<(), String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs();

    match query(
        "INSERT INTO 
                password_resets
                (user_id, reset_token, password, timestamp, valid_until) 
                VALUES (?,?,?,?,?)",
    )
    .bind(user_id)
    .bind(token)
    .bind(password)
    .bind(timestamp as i64)
    .bind(timestamp as i64 + 300)
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!(
            "Failed to insert password reset into the database: {}",
            err
        )),
    }
}

pub async fn get_reset_by_token(
    db: &mut SqliteConnection,
    token: &str,
) -> Result<PasswordResetDB, String> {
    let password_reset: PasswordResetDB =
        match query_as("SELECT * FROM password_resets WHERE reset_token = ?")
            .bind(token)
            .fetch_optional(db)
            .await
        {
            Ok(row) => match row {
                Some(val) => val,
                None => return Err("Password reset not found".to_owned()),
            },
            Err(err) => return Err(format!("Failed to get password reset by token: {}", err)),
        };

    Ok(password_reset)
}

pub async fn get_reset_by_user_id(
    db: &mut SqliteConnection,
    user_id: u32,
) -> Result<PasswordResetDB, String> {
    let password_reset: PasswordResetDB =
        match query_as("SELECT * FROM password_resets WHERE user_id = ?")
            .bind(user_id)
            .fetch_optional(db)
            .await
        {
            Ok(row) => match row {
                Some(val) => val,
                None => return Err("Password reset not found".to_owned()),
            },
            Err(err) => return Err(format!("Failed to get password reset by token: {}", err)),
        };

    Ok(password_reset)
}

pub async fn remove_password_reset_by_user_id(
    db: &mut SqliteConnection,
    user_id: u32,
) -> Result<(), String> {
    match query("DELETE FROM password_resets WHERE user_id = ?")
        .bind(user_id)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to delete passowrd reset: {}", err)),
    }
}

// ███████╗███╗   ███╗ █████╗ ██╗██╗         ██╗   ██╗██████╗ ██████╗  █████╗ ████████╗███████╗
// ██╔════╝████╗ ████║██╔══██╗██║██║         ██║   ██║██╔══██╗██╔══██╗██╔══██╗╚══██╔══╝██╔════╝
// █████╗  ██╔████╔██║███████║██║██║         ██║   ██║██████╔╝██║  ██║███████║   ██║   █████╗
// ██╔══╝  ██║╚██╔╝██║██╔══██║██║██║         ██║   ██║██╔═══╝ ██║  ██║██╔══██║   ██║   ██╔══╝
// ███████╗██║ ╚═╝ ██║██║  ██║██║███████╗    ╚██████╔╝██║     ██████╔╝██║  ██║   ██║   ███████╗
// ╚══════╝╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚══════╝     ╚═════╝ ╚═╝     ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚══════╝

#[derive(Debug, FromRow)]
pub struct EmailUpdateDB {
    pub user_id: u32,
    pub update_token: String,
    pub email: String,
    pub timestamp: i64,
    pub valid_until: i64,
}

pub async fn email_update_in_progress(
    db: &mut SqliteConnection,
    user_id: u32,
) -> Result<bool, String> {
    match query("SELECT user_id FROM email_updates WHERE user_id = ?")
        .bind(user_id)
        .fetch_optional(db)
        .await
    {
        Ok(val) => match val {
            Some(_) => Ok(true),
            None => Ok(false),
        },
        Err(_) => Err("Failed to perform a database query".to_owned()),
    }
}

pub async fn start_email_update(
    db: &mut SqliteConnection,
    user_id: u32,
    email: &str,
    token: &str,
) -> Result<(), String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs();

    match query(
        "INSERT INTO 
                email_updates
                (user_id, update_token, email, timestamp, valid_until) 
                VALUES (?,?,?,?,?)",
    )
    .bind(user_id)
    .bind(token)
    .bind(email)
    .bind(timestamp as i64)
    .bind(timestamp as i64 + 3600)
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!(
            "Failed to insert email update into the database: {}",
            err
        )),
    }
}

pub async fn get_email_update_by_token(
    db: &mut SqliteConnection,
    token: &str,
) -> Result<EmailUpdateDB, String> {
    let password_reset: EmailUpdateDB =
        match query_as("SELECT * FROM email_updates WHERE update_token = ?")
            .bind(token)
            .fetch_optional(db)
            .await
        {
            Ok(row) => match row {
                Some(val) => val,
                None => return Err("Email update not found".to_owned()),
            },
            Err(err) => return Err(format!("Failed to get email update by token: {}", err)),
        };

    Ok(password_reset)
}

pub async fn get_email_update_by_user_id(
    db: &mut SqliteConnection,
    user_id: u32,
) -> Result<EmailUpdateDB, String> {
    let password_reset: EmailUpdateDB =
        match query_as("SELECT * FROM email_updates WHERE user_id = ?")
            .bind(user_id)
            .fetch_optional(db)
            .await
        {
            Ok(row) => match row {
                Some(val) => val,
                None => return Err("Email update not found".to_owned()),
            },
            Err(err) => return Err(format!("Failed to get email update by user id: {}", err)),
        };

    Ok(password_reset)
}

pub async fn remove_email_updates_by_user_id(
    db: &mut SqliteConnection,
    user_id: u32,
) -> Result<(), String> {
    match query("DELETE FROM email_updates WHERE user_id = ?")
        .bind(user_id)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to delete email update: {}", err)),
    }
}
