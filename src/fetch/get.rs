use std::time::{SystemTime, UNIX_EPOCH};

use rocket::{
    http::Status,
    serde::{
        json::{json, Json, Value},
        Deserialize,
    },
};
use rocket_db_pools::Connection;

use crate::DB;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetCharacterData {
    character_id: u32,
}

#[post("/get/character", format = "json", data = "<data>")]
pub async fn get_character(
    mut db: Connection<DB>,
    data: Json<GetCharacterData>,
) -> (Status, Value) {
    let character = match super::get_character(&mut db, data.character_id).await {
        Ok(val) => val,
        Err(err) => return (Status::BadRequest, json!({"error": err})),
    };

    (Status::Ok, json!(character))
}

#[get("/get/pause_state")]
pub async fn get_pause_state(mut db: Connection<DB>) -> (Status, Value) {
    let paused = match super::get_pause(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::BadRequest, json!({"error": err})),
    };

    (Status::Ok, json!(paused.paused))
}

#[get("/get/location_radius")]
pub async fn get_location_radius(mut db: Connection<DB>) -> (Status, Value) {
    let radius = match super::get_location_radius(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::BadRequest, json!({"error": err})),
    };

    (Status::Ok, json!(radius.radius))
}
