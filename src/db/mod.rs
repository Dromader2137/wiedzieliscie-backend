use std::env;

use sqlx::{pool::PoolConnection, query, Sqlite, SqliteConnection};

async fn create_user_table(db: &mut SqliteConnection) -> Result<(), String> {
    if let Ok(var) = env::var("WIEDZIELISCIE_BACKEND_RESET_DB") {
        if var.to_lowercase() == "true" || var == "1" {
            query("DROP TABLE users").execute(&mut *db).await.ok();
        }
    }

    match query(
        "CREATE TABLE users (
        user_id int,
        first_name varchar(255),
        last_name varchar(255),
        email varchar(255),
        password varchar(255),
        gender bool,
        verified bool,
        admin bool
    )",
    )
    .execute(db)
    .await
    {
        Err(err) => {
            if &format!("{}", err)
                == "error returned from database: (code: 1) table users already exists"
            {
                Ok(())
            } else {
                Err(format!("Failed to create users table: {}", err))
            }
        }
        _ => Ok(()),
    }
}

async fn create_verification_table(db: &mut SqliteConnection) -> Result<(), String> {
    if let Ok(var) = env::var("WIEDZIELISCIE_BACKEND_RESET_DB") {
        if var.to_lowercase() == "true" || var == "1" {
            query("DROP TABLE verifications")
                .execute(&mut *db)
                .await
                .ok();
        }
    }

    match query(
        "CREATE TABLE verifications (
        user_id int,
        timestamp int,
        verification_token varchar(255)
    )",
    )
    .execute(db)
    .await
    {
        Err(err) => {
            if &format!("{}", err)
                == "error returned from database: (code: 1) table verifications already exists"
            {
                Ok(())
            } else {
                Err(format!("Failed to create verifications table: {}", err))
            }
        }
        _ => Ok(()),
    }
}

async fn create_session_table(db: &mut SqliteConnection) -> Result<(), String> {
    if let Ok(var) = env::var("WIEDZIELISCIE_BACKEND_RESET_DB") {
        if var.to_lowercase() == "true" || var == "1" {
            query("DROP TABLE sessions").execute(&mut *db).await.ok();
        }
    }

    match query(
        "CREATE TABLE sessions (
        user_id int,
        session_token varchar(255),
        timestamp int,
        valid_until int
    )",
    )
    .execute(db)
    .await
    {
        Err(err) => {
            if &format!("{}", err)
                == "error returned from database: (code: 1) table sessions already exists"
            {
                Ok(())
            } else {
                Err(format!("Failed to create sessions table: {}", err))
            }
        }
        _ => Ok(()),
    }
}

async fn create_password_reser_table(db: &mut SqliteConnection) -> Result<(), String> {
    if let Ok(var) = env::var("WIEDZIELISCIE_BACKEND_RESET_DB") {
        if var.to_lowercase() == "true" || var == "1" {
            query("DROP TABLE password_resets")
                .execute(&mut *db)
                .await
                .ok();
        }
    }

    match query(
        "CREATE TABLE password_resets (
        user_id int,
        reset_token varchar(255),
        password varchar(255), 
        timestamp int,
        valid_until int
    )",
    )
    .execute(db)
    .await
    {
        Err(err) => {
            if &format!("{}", err)
                == "error returned from database: (code: 1) table password_resets already exists"
            {
                Ok(())
            } else {
                Err(format!("Failed to create password_resets table: {}", err))
            }
        }
        _ => Ok(()),
    }
}

async fn create_email_update_table(db: &mut SqliteConnection) -> Result<(), String> {
    if let Ok(var) = env::var("WIEDZIELISCIE_BACKEND_RESET_DB") {
        if var.to_lowercase() == "true" || var == "1" {
            query("DROP TABLE email_updates")
                .execute(&mut *db)
                .await
                .ok();
        }
    }

    match query(
        "CREATE TABLE email_updates (
        user_id int,
        update_token varchar(255),
        email varchar(255), 
        timestamp int,
        valid_until int
    )",
    )
    .execute(db)
    .await
    {
        Err(err) => {
            if &format!("{}", err)
                == "error returned from database: (code: 1) table email_updates already exists"
            {
                Ok(())
            } else {
                Err(format!("Failed to create email_updates table: {}", err))
            }
        }
        _ => Ok(()),
    }
}

async fn create_character_table(db: &mut SqliteConnection) -> Result<(), String> {
    if let Ok(var) = env::var("WIEDZIELISCIE_BACKEND_RESET_DB") {
        if var.to_lowercase() == "true" || var == "1" {
            query("DROP TABLE characters").execute(&mut *db).await.ok();
        }
    }

    match query(
        "CREATE TABLE characters (
        character_id int,
        name varchar(255),
        short_desc varchar(255),
        full_desc varchar(255),
        image varchar(255)
    )",
    )
    .execute(db)
    .await
    {
        Err(err) => {
            if &format!("{}", err)
                == "error returned from database: (code: 1) table characters already exists"
            {
                Ok(())
            } else {
                Err(format!("Failed to create characters table: {}", err))
            }
        }
        _ => Ok(()),
    }
}

async fn create_dialogue_table(db: &mut SqliteConnection) -> Result<(), String> {
    if let Ok(var) = env::var("WIEDZIELISCIE_BACKEND_RESET_DB") {
        if var.to_lowercase() == "true" || var == "1" {
            query("DROP TABLE dialogues").execute(&mut *db).await.ok();
        }
    }

    match query(
        "CREATE TABLE dialogues (
        dialogue_id int,
        quest_id int,
        name varchar(255),
        is_skippable bool
    )",
    )
    .execute(db)
    .await
    {
        Err(err) => {
            if &format!("{}", err)
                == "error returned from database: (code: 1) table dialogues already exists"
            {
                Ok(())
            } else {
                Err(format!("Failed to create dialogues table: {}", err))
            }
        }
        _ => Ok(()),
    }
}

async fn create_dialogue_part_table(db: &mut SqliteConnection) -> Result<(), String> {
    if let Ok(var) = env::var("WIEDZIELISCIE_BACKEND_RESET_DB") {
        if var.to_lowercase() == "true" || var == "1" {
            query("DROP TABLE dialogue_parts")
                .execute(&mut *db)
                .await
                .ok();
        }
    }

    match query(
        "CREATE TABLE dialogue_parts (
        dialogue_id int,
        part_id int,
        character_id int,
        text varchar(65536)
    )",
    )
    .execute(db)
    .await
    {
        Err(err) => {
            if &format!("{}", err)
                == "error returned from database: (code: 1) table dialogue_parts already exists"
            {
                Ok(())
            } else {
                Err(format!("Failed to create dialogue_parts table: {}", err))
            }
        }
        _ => Ok(()),
    }
}

async fn create_task_table(db: &mut SqliteConnection) -> Result<(), String> {
    if let Ok(var) = env::var("WIEDZIELISCIE_BACKEND_RESET_DB") {
        if var.to_lowercase() == "true" || var == "1" {
            query("DROP TABLE tasks").execute(&mut *db).await.ok();
        }
    }

    match query(
        "CREATE TABLE tasks (
        task_id int,
        type varchar(255),
        name varchar(255),
        quest_id int,
        desc varchar(65536),
        min_radius real,
        max_radius real,
        location_to_duplicate int,
        question varchar(65536),
        answers varchar(65536),
        choice_answers varchar(32),
        text_answers varchar(65536)
    )",
    )
    .execute(db)
    .await
    {
        Err(err) => {
            if &format!("{}", err)
                == "error returned from database: (code: 1) table tasks already exists"
            {
                Ok(())
            } else {
                Err(format!("Failed to create tasks table: {}", err))
            }
        }
        _ => Ok(()),
    }
}

async fn create_quest_table(db: &mut SqliteConnection) -> Result<(), String> {
    if let Ok(var) = env::var("WIEDZIELISCIE_BACKEND_RESET_DB") {
        if var.to_lowercase() == "true" || var == "1" {
            query("DROP TABLE quests").execute(&mut *db).await.ok();
        }
    }

    match query(
        "CREATE TABLE quests (
        quest_id int,
        desc varchar(65536),
        unlocks varchar(65536),
        points int,
        coins int,
        rewards varchar(65536)
    )",
    )
    .execute(db)
    .await
    {
        Err(err) => {
            if &format!("{}", err)
                == "error returned from database: (code: 1) table quests already exists"
            {
                Ok(())
            } else {
                Err(format!("Failed to create quests table: {}", err))
            }
        }
        _ => Ok(()),
    }
}

async fn create_quest_stage_table(db: &mut SqliteConnection) -> Result<(), String> {
    if let Ok(var) = env::var("WIEDZIELISCIE_BACKEND_RESET_DB") {
        if var.to_lowercase() == "true" || var == "1" {
            query("DROP TABLE quest_stages")
                .execute(&mut *db)
                .await
                .ok();
        }
    }

    match query(
        "CREATE TABLE quest_stages (
        quest_id int,
        stage_id int,
        task_id int,
        dialogue_id int
    )",
    )
    .execute(db)
    .await
    {
        Err(err) => {
            if &format!("{}", err)
                == "error returned from database: (code: 1) table quest_stages already exists"
            {
                Ok(())
            } else {
                Err(format!("Failed to create quest_stages table: {}", err))
            }
        }
        _ => Ok(()),
    }
}

pub async fn create_tables(mut db: PoolConnection<Sqlite>) {
    create_user_table(&mut db).await.unwrap();
    create_verification_table(&mut db).await.unwrap();
    create_session_table(&mut db).await.unwrap();
    create_password_reser_table(&mut db).await.unwrap();
    create_email_update_table(&mut db).await.unwrap();
    create_character_table(&mut db).await.unwrap();
    create_dialogue_table(&mut db).await.unwrap();
    create_dialogue_part_table(&mut db).await.unwrap();
    create_task_table(&mut db).await.unwrap();
    create_quest_table(&mut db).await.unwrap();
    create_quest_stage_table(&mut db).await.unwrap();
}
