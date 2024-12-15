use sqlx::{prelude::FromRow, query, query_as, Row, SqliteConnection};

//  █████╗ ██████╗ ███╗   ███╗██╗███╗   ██╗    ███████╗██╗   ██╗███╗   ██╗ ██████╗████████╗██╗ ██████╗ ███╗   ██╗███████╗
// ██╔══██╗██╔══██╗████╗ ████║██║████╗  ██║    ██╔════╝██║   ██║████╗  ██║██╔════╝╚══██╔══╝██║██╔═══██╗████╗  ██║██╔════╝
// ███████║██║  ██║██╔████╔██║██║██╔██╗ ██║    █████╗  ██║   ██║██╔██╗ ██║██║        ██║   ██║██║   ██║██╔██╗ ██║███████╗
// ██╔══██║██║  ██║██║╚██╔╝██║██║██║╚██╗██║    ██╔══╝  ██║   ██║██║╚██╗██║██║        ██║   ██║██║   ██║██║╚██╗██║╚════██║
// ██║  ██║██████╔╝██║ ╚═╝ ██║██║██║ ╚████║    ██║     ╚██████╔╝██║ ╚████║╚██████╗   ██║   ██║╚██████╔╝██║ ╚████║███████║
// ╚═╝  ╚═╝╚═════╝ ╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝    ╚═╝      ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝   ╚═╝   ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚══════╝

//  ██████╗██╗  ██╗ █████╗ ██████╗  █████╗  ██████╗████████╗███████╗██████╗
// ██╔════╝██║  ██║██╔══██╗██╔══██╗██╔══██╗██╔════╝╚══██╔══╝██╔════╝██╔══██╗
// ██║     ███████║███████║██████╔╝███████║██║        ██║   █████╗  ██████╔╝
// ██║     ██╔══██║██╔══██║██╔══██╗██╔══██║██║        ██║   ██╔══╝  ██╔══██╗
// ╚██████╗██║  ██║██║  ██║██║  ██║██║  ██║╚██████╗   ██║   ███████╗██║  ██║
//  ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝   ╚═╝   ╚══════╝╚═╝  ╚═╝

#[derive(Debug, FromRow)]
pub struct Character {
    pub character_id: u32,
    pub name: String,
    pub short_desc: String,
    pub full_desc: String,
    pub image: String,
}

pub async fn next_character_id(db: &mut SqliteConnection) -> Result<u32, String> {
    match query("SELECT MAX(character_id) FROM characters")
        .fetch_optional(db)
        .await
    {
        Ok(val) => match val {
            Some(row) => match row.try_get::<u32, _>(0) {
                Ok(id) => Ok(id + 1),
                Err(_) => Err("Database error".to_owned()),
            },
            None => Ok(1),
        },
        Err(_) => Err("Failed to perform a database query".to_owned()),
    }
}

pub async fn create_character(
    db: &mut SqliteConnection,
    id: u32,
    name: &str,
    short_desc: &str,
    full_desc: &str,
    image: &str,
) -> Result<(), String> {
    match query(
        "INSERT INTO
        characters
        (character_id, name, short_desc, full_desc, image)
        VALUES (?,?,?,?,?)",
    )
    .bind(id)
    .bind(name)
    .bind(short_desc)
    .bind(full_desc)
    .bind(image)
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to create user: {}", err)),
    }
}

pub async fn delete_character(db: &mut SqliteConnection, id: u32) -> Result<(), String> {
    match query("DELETE FROM characters WHERE character_id = ?")
        .bind(id)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to delete character: {}", err)),
    }
}

pub async fn get_all_characters(db: &mut SqliteConnection) -> Result<Vec<Character>, String> {
    match query_as::<_, Character>("SELECT * FROM characters")
        .fetch_all(db)
        .await
    {
        Ok(val) => Ok(val),
        Err(err) => return Err(format!("Failed to get characters: {}", err)),
    }
}

// ██████╗ ██╗ █████╗ ██╗      ██████╗  ██████╗ ██╗   ██╗███████╗
// ██╔══██╗██║██╔══██╗██║     ██╔═══██╗██╔════╝ ██║   ██║██╔════╝
// ██║  ██║██║███████║██║     ██║   ██║██║  ███╗██║   ██║█████╗
// ██║  ██║██║██╔══██║██║     ██║   ██║██║   ██║██║   ██║██╔══╝
// ██████╔╝██║██║  ██║███████╗╚██████╔╝╚██████╔╝╚██████╔╝███████╗
// ╚═════╝ ╚═╝╚═╝  ╚═╝╚══════╝ ╚═════╝  ╚═════╝  ╚═════╝ ╚══════╝

#[derive(Debug, FromRow)]
pub struct Dialogue {
    pub dialogue_id: u32,
    pub quest_id: Option<u32>,
    pub name: String,
    pub is_skippable: bool,
}

#[derive(Debug, FromRow)]
pub struct DialoguePart {
    pub dialogue_id: u32,
    pub part_id: u32,
    pub text: String,
}

pub async fn next_dialogue_id(db: &mut SqliteConnection) -> Result<u32, String> {
    match query("SELECT MAX(dialogue_id) FROM dialogues")
        .fetch_optional(db)
        .await
    {
        Ok(val) => match val {
            Some(row) => match row.try_get::<u32, _>(0) {
                Ok(id) => Ok(id + 1),
                Err(_) => Err("Database error".to_owned()),
            },
            None => Ok(1),
        },
        Err(_) => Err("Failed to perform a database query".to_owned()),
    }
}

pub async fn create_dialogue(
    db: &mut SqliteConnection,
    id: u32,
    quest_id: Option<u32>,
    name: &str,
    is_skippable: bool,
) -> Result<(), String> {
    match query(
        "INSERT INTO
        dialogues
        (dialogue_id, quest_id, name, is_skippable)
        VALUES (?,?,?,?,?)",
    )
    .bind(id)
    .bind(quest_id)
    .bind(name)
    .bind(is_skippable)
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to create dialogue: {}", err)),
    }
}

pub async fn delete_dialogue(db: &mut SqliteConnection, id: u32) -> Result<(), String> {
    match query("DELETE FROM dialogues WHERE dialogue_id = ?")
        .bind(id)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to delete dialogue: {}", err)),
    }
}

pub async fn get_all_dialogues(db: &mut SqliteConnection) -> Result<Vec<Dialogue>, String> {
    match query_as::<_, Dialogue>("SELECT * FROM dialogues")
        .fetch_all(db)
        .await
    {
        Ok(val) => Ok(val),
        Err(err) => return Err(format!("Failed to get dialogues: {}", err)),
    }
}

pub async fn get_unused_dialogues(db: &mut SqliteConnection) -> Result<Vec<Dialogue>, String> {
    match query_as::<_, Dialogue>("SELECT * FROM dialogues WHERE quest_id is Null")
        .fetch_all(db)
        .await
    {
        Ok(val) => Ok(val),
        Err(err) => return Err(format!("Failed to get unused dialogues: {}", err)),
    }
}

pub async fn get_dialogue_parts(
    db: &mut SqliteConnection,
    dialogue_id: u32,
) -> Result<Vec<DialoguePart>, String> {
    match query_as::<_, DialoguePart>("SELECT * FROM dialogue_parts WHERE dialogue_id = ?")
        .bind(dialogue_id)
        .fetch_all(db)
        .await
    {
        Ok(mut val) => {
            val.sort_by(|a, b| a.part_id.cmp(&b.part_id));
            Ok(val)
        }
        Err(err) => return Err(format!("Failed to get unused dialogues: {}", err)),
    }
}
