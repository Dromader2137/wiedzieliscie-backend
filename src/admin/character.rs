use rocket::{
    http::Status,
    serde::{
        json::{json, Json, Value},
        Deserialize,
    },
};
use rocket_db_pools::Connection;

use crate::{util::check_authorized_admin, DB};

use super::{create_character, delete_character, get_all_characters, next_character_id};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CharacterAddData<'r> {
    jwt: &'r str,
    name: &'r str,
    short_description: &'r str,
    full_description: &'r str,
    image: &'r str,
}

#[post("/admin/characters/add", format = "json", data = "<data>")]
pub async fn admin_characters_add(
    mut db: Connection<DB>,
    data: Json<CharacterAddData<'_>>,
) -> (Status, Value) {
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
    }

    let character_id = match next_character_id(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    if let Err(err) = create_character(
        &mut db,
        character_id,
        data.name,
        data.short_description,
        data.full_description,
        data.image,
    )
    .await
    {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({"character_id": character_id}))
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CharacterDeleteData<'r> {
    jwt: &'r str,
    character_id: u32,
}

#[post("/admin/characters/delete", format = "json", data = "<data>")]
pub async fn admin_characters_delete(
    mut db: Connection<DB>,
    data: Json<CharacterDeleteData<'_>>,
) -> (Status, Value) {
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
    }

    if let Err(err) = delete_character(&mut db, data.character_id).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CharacterGetData<'r> {
    jwt: &'r str,
}

#[post("/admin/characters/get", format = "json", data = "<data>")]
pub async fn admin_characters_get(
    mut db: Connection<DB>,
    data: Json<CharacterGetData<'_>>,
) -> (Status, Value) {
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
    }

    let characters = match get_all_characters(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::InternalServerError, json!({"error": err})),
    };

    (Status::Ok, json!(characters))
}
