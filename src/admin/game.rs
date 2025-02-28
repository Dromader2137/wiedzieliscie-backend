use std::time::{SystemTime, UNIX_EPOCH};

use rocket::{
    http::Status,
    serde::{
        json::{json, Json, Value},
        Deserialize,
    },
};
use rocket_db_pools::Connection;

use crate::{
    user::{get_session_by_token, get_user_by_id, jwt::verify_token},
    DB,
};

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
    let claims = match verify_token(data.jwt) {
        Ok(val) => val.claims,
        Err(_) => return (Status::BadRequest, json!({"error": "invalid token"})),
    };

    let user_id = claims.uid;
    let session_token = claims.token;

    let user = match get_user_by_id(&mut db, user_id).await {
        Ok(val) => val,
        Err(err) => return (Status::BadRequest, json!({"error": err})),
    };

    if !user.admin {
        return (
            Status::BadRequest,
            json!({"error": "account_id and token are different"}),
        );
    }

    let sessions = match get_session_by_token(&mut db, &session_token).await {
        Ok(val) => val,
        Err(err) => return (Status::BadRequest, json!({"error": err})),
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs() as i64;

    if timestamp > sessions.valid_until {
        return (Status::BadRequest, json!({"error": "token expired"}));
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
    let claims = match verify_token(data.jwt) {
        Ok(val) => val.claims,
        Err(_) => return (Status::BadRequest, json!({"error": "invalid token"})),
    };

    let user_id = claims.uid;
    let session_token = claims.token;

    let user = match get_user_by_id(&mut db, user_id).await {
        Ok(val) => val,
        Err(err) => return (Status::BadRequest, json!({"error": err})),
    };

    if !user.admin {
        return (
            Status::BadRequest,
            json!({"error": "account_id and token are different"}),
        );
    }

    let sessions = match get_session_by_token(&mut db, &session_token).await {
        Ok(val) => val,
        Err(err) => return (Status::BadRequest, json!({"error": err})),
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs() as i64;

    if timestamp > sessions.valid_until {
        return (Status::BadRequest, json!({"error": "token expired"}));
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
    let claims = match verify_token(data.jwt) {
        Ok(val) => val.claims,
        Err(_) => return (Status::BadRequest, json!({"error": "invalid token"})),
    };

    let user_id = claims.uid;
    let session_token = claims.token;

    let user = match get_user_by_id(&mut db, user_id).await {
        Ok(val) => val,
        Err(err) => return (Status::BadRequest, json!({"error": err})),
    };

    if !user.admin {
        return (
            Status::BadRequest,
            json!({"error": "account_id and token are different"}),
        );
    }

    let sessions = match get_session_by_token(&mut db, &session_token).await {
        Ok(val) => val,
        Err(err) => return (Status::BadRequest, json!({"error": err})),
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs() as i64;

    if timestamp > sessions.valid_until {
        return (Status::BadRequest, json!({"error": "token expired"}));
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
    let claims = match verify_token(data.jwt) {
        Ok(val) => val.claims,
        Err(_) => return (Status::BadRequest, json!({"error": "invalid token"})),
    };

    let user_id = claims.uid;
    let session_token = claims.token;

    let user = match get_user_by_id(&mut db, user_id).await {
        Ok(val) => val,
        Err(err) => return (Status::BadRequest, json!({"error": err})),
    };

    if !user.admin {
        return (
            Status::BadRequest,
            json!({"error": "account_id and token are different"}),
        );
    }

    let sessions = match get_session_by_token(&mut db, &session_token).await {
        Ok(val) => val,
        Err(err) => return (Status::BadRequest, json!({"error": err})),
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time")
        .as_secs() as i64;

    if timestamp > sessions.valid_until {
        return (Status::BadRequest, json!({"error": "token expired"}));
    }

    if let Err(err) = game_set_location_radius(&mut db, data.distance).await {
        return (Status::BadRequest, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}
