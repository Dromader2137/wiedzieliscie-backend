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
    add_choice_task, add_location_task, add_text_task, get_tasks, get_tasks_unused, next_task_id,
};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LocationTaskAddData<'r> {
    jwt: &'r str,
    quest_id: Option<u32>,
    name: &'r str,
    desc: Option<&'r str>,
    min_radius: f32,
    max_radius: f32,
    location_to_duplicate: Option<u32>,
}

#[post("/admin/tasks/location/add", format = "json", data = "<data>")]
pub async fn admin_tasks_location_add(
    mut db: Connection<DB>,
    data: Json<LocationTaskAddData<'_>>,
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

    let task_id = match next_task_id(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    if let Err(err) = add_location_task(
        &mut db,
        task_id,
        data.name,
        data.quest_id,
        data.desc,
        data.min_radius,
        data.max_radius,
        data.location_to_duplicate,
    )
    .await
    {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({"task_id": task_id}))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MultipleChoiceTaskAddData<'r> {
    jwt: &'r str,
    quest_id: Option<u32>,
    name: &'r str,
    desc: Option<&'r str>,
    question: &'r str,
    answers: Vec<&'r str>,
    correct_answers: Vec<u32>,
}

#[post("/admin/tasks/multiple_choice/add", format = "json", data = "<data>")]
pub async fn admin_tasks_multiple_choice_add(
    mut db: Connection<DB>,
    data: Json<MultipleChoiceTaskAddData<'_>>,
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

    let task_id = match next_task_id(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    if let Err(err) = add_choice_task(
        &mut db,
        task_id,
        data.name,
        data.quest_id,
        data.desc,
        data.question,
        &data.answers,
        &data.correct_answers,
    )
    .await
    {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({"task_id": task_id}))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TextTaskAddData<'r> {
    jwt: &'r str,
    quest_id: Option<u32>,
    name: &'r str,
    desc: Option<&'r str>,
    question: &'r str,
    correct_answers: Vec<&'r str>,
}

#[post("/admin/tasks/text_answer/add", format = "json", data = "<data>")]
pub async fn admin_tasks_text_answer_add(
    mut db: Connection<DB>,
    data: Json<TextTaskAddData<'_>>,
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

    let task_id = match next_task_id(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    if let Err(err) = add_text_task(
        &mut db,
        task_id,
        data.name,
        data.quest_id,
        data.desc,
        data.question,
        &data.correct_answers,
    )
    .await
    {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({"task_id": task_id}))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TaskGetData<'r> {
    jwt: &'r str,
}

#[post("/admin/tasks/get", format = "json", data = "<data>")]
pub async fn admin_tasks_get(
    mut db: Connection<DB>,
    data: Json<TaskGetData<'_>>,
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

    let tasks = match get_tasks(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    (Status::Ok, json!(tasks))
}

#[post("/admin/tasks/get/unused", format = "json", data = "<data>")]
pub async fn admin_tasks_get_unused(
    mut db: Connection<DB>,
    data: Json<TaskGetData<'_>>,
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

    let tasks = match get_tasks_unused(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    (Status::Ok, json!(tasks))
}
