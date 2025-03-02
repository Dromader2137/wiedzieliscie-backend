use rocket::{
    http::Status,
    serde::{
        json::{json, Json, Value},
        Deserialize,
    },
};
use rocket_db_pools::Connection;

use crate::{util::check_authorized_admin, DB};

use super::{
    add_quest_stage, change_quest_stage_id_back, change_quest_stage_id_forward, create_quest,
    delete_quest, delete_quest_stage, get_all_quest_stages, get_all_quests, get_quest_by_id,
    next_quest_id, next_quest_stage_id, QuestStageContent,
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
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
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
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
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
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
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
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
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
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
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
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
    }

    let quest_stage_id = match next_quest_stage_id(&mut db, data.quest_id).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    if data.position == quest_stage_id - 1 {
        return (
            Status::BadRequest,
            json!({"error": "position can't be max"}),
        );
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
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
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
}

#[post("/admin/quests/get", format = "json", data = "<data>")]
pub async fn admin_quests_get(
    mut db: Connection<DB>,
    data: Json<QuestGetData<'_>>,
) -> (Status, Value) {
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
    }

    let quests = match get_all_quests(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    (Status::Ok, json!(quests))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct QuestDuplicateData<'r> {
    jwt: &'r str,
    quest_id: u32,
}

#[post("/admin/quests/duplicate", format = "json", data = "<data>")]
pub async fn admin_quests_duplicate(
    mut db: Connection<DB>,
    data: Json<QuestDuplicateData<'_>>,
) -> (Status, Value) {
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
    }

    let quest_id = match next_quest_id(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    let quest = match get_quest_by_id(&mut db, data.quest_id).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    if let Err(err) = create_quest(
        &mut db,
        quest_id,
        &quest.name,
        &quest.desc,
        &quest.unlocks,
        quest.points,
        quest.points,
        &quest.rewards,
    )
    .await
    {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}
