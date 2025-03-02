use rocket::serde::Serialize;
use sqlx::{prelude::FromRow, query, query_as, SqliteConnection};

pub mod report;

pub async fn add_error(
    db: &mut SqliteConnection,
    title: &str,
    message: &str,
) -> Result<(), String> {
    match query(
        "INSERT INTO 
                error_report 
                (title, message)
                VALUES (?,?)",
    )
    .bind(title)
    .bind(message)
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!(
            "Failed to insert error report into the database: {}",
            err
        )),
    }
}

pub async fn add_suggestion(
    db: &mut SqliteConnection,
    title: &str,
    message: &str,
) -> Result<(), String> {
    match query(
        "INSERT INTO 
                suggestion 
                (title, message)
                VALUES (?,?)",
    )
    .bind(title)
    .bind(message)
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!(
            "Failed to insert suggestion into the database: {}",
            err
        )),
    }
}

#[derive(Debug, FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorReport {
    title: String,
    message: String,
}

pub async fn get_reports(db: &mut SqliteConnection) -> Result<Vec<ErrorReport>, String> {
    match query_as::<_, ErrorReport>("SELECT * FROM error_report")
        .fetch_all(db)
        .await
    {
        Ok(val) => Ok(val),
        Err(err) => return Err(err.to_string()),
    }
}
