use rocket::{
    http::Status,
    serde::{
        json::{json, Json, Value},
        Deserialize,
    },
};
use rocket_db_pools::Connection;

use crate::{util::check_authorized_admin, DB};

use super::{game_set_location_radius, game_set_state, game_set_tutorial};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GamePauseData<'r> {
    jwt: &'r str,
}

#[post("/admin/game/pause", format = "json", data = "<data>")]
pub async fn admin_game_pause(
    mut db: Connection<DB>,
    data: Json<GamePauseData<'_>>,
) -> (Status, Value) {
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
    }

    if let Err(err) = game_set_state(&mut db, true).await {
        return (Status::BadRequest, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GameUnpauseData<'r> {
    jwt: &'r str,
}

#[post("/admin/game/unpause", format = "json", data = "<data>")]
pub async fn admin_game_unpause(
    mut db: Connection<DB>,
    data: Json<GameUnpauseData<'_>>,
) -> (Status, Value) {
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
    }

    if let Err(err) = game_set_state(&mut db, false).await {
        return (Status::BadRequest, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GameSetTutorialData<'r> {
    jwt: &'r str,
    quest_id: u32,
}

#[post("/admin/quests/select_tutorial", format = "json", data = "<data>")]
pub async fn admin_quests_select_tutorial(
    mut db: Connection<DB>,
    data: Json<GameSetTutorialData<'_>>,
) -> (Status, Value) {
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
    }

    if let Err(err) = game_set_tutorial(&mut db, data.quest_id).await {
        return (Status::BadRequest, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GameSetLocationRadiusData<'r> {
    jwt: &'r str,
    distance: f32,
}

#[post("/admin/game/set_location_radius", format = "json", data = "<data>")]
pub async fn admin_game_set_location_radius(
    mut db: Connection<DB>,
    data: Json<GameSetLocationRadiusData<'_>>,
) -> (Status, Value) {
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
    }

    if let Err(err) = game_set_location_radius(&mut db, data.distance).await {
        return (Status::BadRequest, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}
