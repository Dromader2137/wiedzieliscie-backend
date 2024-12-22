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

use super::{
    create_dialogue, delete_dialogue, delete_dialogue_parts, get_all_dialogues,
    get_unused_dialogues, next_dialogue_id, set_dialogue_parts,
};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DialogueAddData<'r> {
    jwt: &'r str,
    quest_id: Option<u32>,
    name: &'r str,
    is_skippable: bool,
    parts: Vec<(u32, &'r str)>,
}

#[post("/admin/dialogues/add", format = "json", data = "<data>")]
pub async fn admin_dialogues_add(
    mut db: Connection<DB>,
    data: Json<DialogueAddData<'_>>,
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

    let dialogue_id = match next_dialogue_id(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    if let Err(err) = create_dialogue(
        &mut db,
        dialogue_id,
        data.quest_id,
        data.name,
        data.is_skippable,
    )
    .await
    {
        return (Status::InternalServerError, json!({"error": err}));
    }

    if let Err(err) = set_dialogue_parts(&mut db, dialogue_id, &data.parts).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({"dialogue_id": dialogue_id}))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DialogueDeleteData<'r> {
    jwt: &'r str,
    dialogue_id: u32,
}

#[post("/admin/dialogues/delete", format = "json", data = "<data>")]
pub async fn admin_dialogues_delete(
    mut db: Connection<DB>,
    data: Json<DialogueDeleteData<'_>>,
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

    if let Err(err) = delete_dialogue(&mut db, data.dialogue_id).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    if let Err(err) = delete_dialogue_parts(&mut db, data.dialogue_id).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DialogueGetData<'r> {
    jwt: &'r str,
}

#[post("/admin/dialogues/get", format = "json", data = "<data>")]
pub async fn admin_dialogues_get(
    mut db: Connection<DB>,
    data: Json<DialogueGetData<'_>>,
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

    let dialogues = match get_all_dialogues(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    (Status::Ok, json!(dialogues))
}

#[post("/admin/dialogues/get/unused", format = "json", data = "<data>")]
pub async fn admin_dialogues_get_unused(
    mut db: Connection<DB>,
    data: Json<DialogueGetData<'_>>,
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

    let dialogues = match get_unused_dialogues(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    (Status::Ok, json!(dialogues))
}
