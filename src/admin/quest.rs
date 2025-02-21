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
    add_quest_stage, change_quest_stage_id_back, change_quest_stage_id_forward, create_quest,
    delete_quest, delete_quest_stage, get_all_quest_stages, get_all_quests, next_quest_id,
    next_quest_stage_id, QuestStageContent,
};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct QuestAddData<'r> {
    jwt: &'r str,
    name: &'r str,
    description: &'r str,
    unlocks: Vec<u32>,
    points: u32,
    coins: u32,
    rewards: Vec<u32>,
}

#[post("/admin/quests/add", format = "json", data = "<data>")]
pub async fn admin_quests_add(
    mut db: Connection<DB>,
    data: Json<QuestAddData<'_>>,
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

    let quest_id = match next_quest_id(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    if let Err(err) = create_quest(
        &mut db,
        quest_id,
        data.name,
        data.description,
        &data.unlocks,
        data.points,
        data.coins,
        &data.rewards,
    )
    .await
    {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct QuestStageAddData<'r> {
    jwt: &'r str,
    quest_id: u32,
    task_id: Option<u32>,
    dialogue_id: Option<u32>,
}

#[post("/admin/quests/stages/add", format = "json", data = "<data>")]
pub async fn admin_quests_stages_add(
    mut db: Connection<DB>,
    data: Json<QuestStageAddData<'_>>,
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

    let quest_stage_id = match next_quest_stage_id(&mut db, data.quest_id).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    if let Some(task_id) = data.task_id {
        if let Err(err) = add_quest_stage(
            &mut db,
            data.quest_id,
            quest_stage_id,
            QuestStageContent::Task(task_id),
        )
        .await
        {
            return (Status::InternalServerError, json!({"error": err}));
        }
    } else if let Some(dialogue_id) = data.dialogue_id {
        if let Err(err) = add_quest_stage(
            &mut db,
            data.quest_id,
            quest_stage_id,
            QuestStageContent::Dialogue(dialogue_id),
        )
        .await
        {
            return (Status::InternalServerError, json!({"error": err}));
        }
    }

    (Status::Ok, json!({}))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct QuestStageDeleteData<'r> {
    jwt: &'r str,
    quest_id: u32,
    position: u32,
}

#[post("/admin/quests/stages/delete", format = "json", data = "<data>")]
pub async fn admin_quests_stages_delete(
    mut db: Connection<DB>,
    data: Json<QuestStageDeleteData<'_>>,
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

    if let Err(err) = delete_quest_stage(&mut db, data.quest_id, data.position).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct QuestStageGetData<'r> {
    jwt: &'r str,
    quest_id: u32,
}

#[post("/admin/quests/stages/get", format = "json", data = "<data>")]
pub async fn admin_quests_stages_get(
    mut db: Connection<DB>,
    data: Json<QuestStageGetData<'_>>,
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

    let quests = match get_all_quest_stages(&mut db, data.quest_id).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    (Status::Ok, json!(quests))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct QuestStageMoveBackData<'r> {
    jwt: &'r str,
    quest_id: u32,
    position: u32,
}

#[post("/admin/quests/stages/move_back", format = "json", data = "<data>")]
pub async fn admin_quests_stages_move_back(
    mut db: Connection<DB>,
    data: Json<QuestStageMoveBackData<'_>>,
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

    if data.position == 0 {
        return (Status::BadRequest, json!({"error": "position can't be 0"}));
    }

    if let Err(err) = change_quest_stage_id_back(&mut db, data.quest_id, data.position).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct QuestStageMoveForwardData<'r> {
    jwt: &'r str,
    quest_id: u32,
    position: u32,
}

#[post("/admin/quests/stages/move_forward", format = "json", data = "<data>")]
pub async fn admin_quests_stages_move_forward(
    mut db: Connection<DB>,
    data: Json<QuestStageMoveForwardData<'_>>,
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

    if data.position == 0 {
        return (Status::BadRequest, json!({"error": "position can't be 0"}));
    }

    if let Err(err) = change_quest_stage_id_forward(&mut db, data.quest_id, data.position).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct QuestDeleteData<'r> {
    jwt: &'r str,
    quest_id: u32,
}

#[post("/admin/quests/delete", format = "json", data = "<data>")]
pub async fn admin_quests_delete(
    mut db: Connection<DB>,
    data: Json<QuestDeleteData<'_>>,
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

    if let Err(err) = delete_quest(&mut db, data.quest_id).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct QuestGetData<'r> {
    jwt: &'r str,
    quest_id: u32,
}

#[post("/admin/quests/get", format = "json", data = "<data>")]
pub async fn admin_quests_get(
    mut db: Connection<DB>,
    data: Json<QuestGetData<'_>>,
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

    let quests = match get_all_quests(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    (Status::Ok, json!(quests))
}
