use rocket::serde::Serialize;
use sqlx::{prelude::FromRow, query_as, SqliteConnection};

pub mod get;

use crate::admin::Character;

pub async fn get_character(db: &mut SqliteConnection, id: u32) -> Result<Character, String> {
    match query_as::<_, Character>("SELECT * FROM characters WHERE character_id = ?")
        .bind(id)
        .fetch_one(db)
        .await
    {
        Ok(val) => Ok(val),
        Err(err) => Err(err.to_string()),
    }
}

#[derive(Debug, Serialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct PauseState {
    paused: bool,
}

pub async fn get_pause(db: &mut SqliteConnection) -> Result<PauseState, String> {
    match query_as::<_, PauseState>("SELECT paused FROM game")
        .fetch_one(db)
        .await
    {
        Ok(val) => Ok(val),
        Err(err) => Err(err.to_string()),
    }
}

#[derive(Debug, Serialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct LocationRadius {
    radius: f32,
}

pub async fn get_location_radius(db: &mut SqliteConnection) -> Result<LocationRadius, String> {
    match query_as::<_, LocationRadius>("SELECT location_radius FROM game")
        .fetch_one(db)
        .await
    {
        Ok(val) => Ok(val),
        Err(err) => Err(err.to_string()),
    }
}
