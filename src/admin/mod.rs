pub mod character;
pub mod dialogue;

use rocket::serde::Serialize;
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

#[derive(Debug, FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
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
        Err(err) => Err(format!("Failed to get characters: {}", err)),
    }
}

// ██████╗ ██╗ █████╗ ██╗      ██████╗  ██████╗ ██╗   ██╗███████╗
// ██╔══██╗██║██╔══██╗██║     ██╔═══██╗██╔════╝ ██║   ██║██╔════╝
// ██║  ██║██║███████║██║     ██║   ██║██║  ███╗██║   ██║█████╗
// ██║  ██║██║██╔══██║██║     ██║   ██║██║   ██║██║   ██║██╔══╝
// ██████╔╝██║██║  ██║███████╗╚██████╔╝╚██████╔╝╚██████╔╝███████╗
// ╚═════╝ ╚═╝╚═╝  ╚═╝╚══════╝ ╚═════╝  ╚═════╝  ╚═════╝ ╚══════╝

#[derive(Debug, FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Dialogue {
    pub dialogue_id: u32,
    pub quest_id: Option<u32>,
    pub name: String,
    pub is_skippable: bool,
}

#[derive(Debug, FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DialoguePart {
    pub dialogue_id: u32,
    pub part_id: u32,
    pub character_id: u32,
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
        VALUES (?,?,?,?)",
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
        Err(err) => Err(format!("Failed to get dialogues: {}", err)),
    }
}

pub async fn get_unused_dialogues(db: &mut SqliteConnection) -> Result<Vec<Dialogue>, String> {
    match query_as::<_, Dialogue>("SELECT * FROM dialogues WHERE quest_id is Null")
        .fetch_all(db)
        .await
    {
        Ok(val) => Ok(val),
        Err(err) => Err(format!("Failed to get unused dialogues: {}", err)),
    }
}

pub async fn set_dialogue_parts(
    db: &mut SqliteConnection,
    dialogue_id: u32,
    dialogue_parts: &Vec<(u32, &str)>,
) -> Result<(), String> {
    if dialogue_parts.is_empty() {
        return Err("Empty dialogue_parts not allowed".to_string());
    }

    let mut insertion_query =
        "INSERT INTO dialogue_parts (dialogue_id, part_id, character_id, text) VALUES ".to_string();
    for (part_id, (part_user, part_text)) in dialogue_parts.iter().enumerate() {
        insertion_query += &format!(
            "({}, {}, {}, \"{}\"),",
            dialogue_id, part_id, part_user, part_text
        );
    }
    insertion_query.pop().unwrap();

    match query(&insertion_query).execute(db).await {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to set dialogue parts: {}", err)),
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
        Err(err) => Err(format!("Failed to get unused dialogues: {}", err)),
    }
}

pub async fn delete_dialogue_parts(db: &mut SqliteConnection, id: u32) -> Result<(), String> {
    match query("DELETE FROM dialogue_parts WHERE dialogue_id = ?")
        .bind(id)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to delete dialogue parts: {}", err)),
    }
}

// ████████╗ █████╗ ███████╗██╗  ██╗
// ╚══██╔══╝██╔══██╗██╔════╝██║ ██╔╝
//    ██║   ███████║███████╗█████╔╝
//    ██║   ██╔══██║╚════██║██╔═██╗
//    ██║   ██║  ██║███████║██║  ██╗
//    ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝

#[derive(Debug, FromRow)]
pub struct LocationTask {
    pub task_id: u32,
    pub name: String,
    pub quest_id: Option<u32>,
    pub desc: String,
    pub min_radius: f32,
    pub max_radius: f32,
    pub location_to_duplicate: Option<u32>,
}

#[derive(Debug, FromRow)]
pub struct ChoiceTask {
    pub task_id: u32,
    pub name: String,
    pub quest_id: Option<u32>,
    pub desc: String,
    pub question: String,
    pub answers: Vec<String>,
    pub choice_answers: Vec<u32>,
}

#[derive(Debug, FromRow)]
pub struct TextTask {
    pub task_id: u32,
    pub name: String,
    pub quest_id: Option<u32>,
    pub desc: String,
    pub question: String,
    pub text_answers: Vec<String>,
}

#[derive(Debug)]
pub enum Task {
    Location(LocationTask),
    Choice(ChoiceTask),
    Text(TextTask),
    Invalid(String),
}

pub async fn next_task_id(db: &mut SqliteConnection) -> Result<u32, String> {
    match query("SELECT MAX(task_id) FROM tasks")
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

#[allow(clippy::too_many_arguments)]
pub async fn add_location_task(
    db: &mut SqliteConnection,
    task_id: u32,
    name: &str,
    quest_id: Option<u32>,
    desc: &str,
    min_radius: f32,
    max_radius: f32,
    location_to_duplicate: Option<u32>,
) -> Result<(), String> {
    match query(
        "INSERT INTO tasks
        (task_id, type, name, quest_id, desc, min_radius, max_radius, location_to_duplicate)
        VALUES
        (?,\'location\',?,?,?,?,?,?)",
    )
    .bind(task_id)
    .bind(name)
    .bind(quest_id)
    .bind(desc)
    .bind(min_radius)
    .bind(max_radius)
    .bind(location_to_duplicate)
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to add location task: {}", err)),
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn add_choice_task(
    db: &mut SqliteConnection,
    task_id: u32,
    name: &str,
    quest_id: Option<u32>,
    desc: &str,
    question: &str,
    answers: Vec<&str>,
    correct_answers: Vec<u32>,
) -> Result<(), String> {
    let mut ans = [false; 32];
    let mut ans_str = String::new();

    for id in correct_answers.iter() {
        if *id >= 32 {
            return Err("Questions with more than 32 answers are not allowed".to_string());
        }
        ans[*id as usize] = true;
    }

    for i in ans.iter() {
        ans_str.push(if *i { '1' } else { '0' });
    }

    let answers = answers
        .iter()
        .map(|x| format!("{}\n", x))
        .fold("".to_string(), |acc, x| format!("{}{}", acc, x));

    match query(
        "INSERT INTO tasks
        (task_id, type, name, quest_id, desc, question, answers, choice_answers)
        VALUES
        (?,\'choice\',?,?,?,?,?,?)",
    )
    .bind(task_id)
    .bind(name)
    .bind(quest_id)
    .bind(desc)
    .bind(question)
    .bind(answers)
    .bind(ans_str)
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to add choice task: {}", err)),
    }
}

pub async fn add_text_task(
    db: &mut SqliteConnection,
    task_id: u32,
    name: &str,
    quest_id: Option<u32>,
    desc: &str,
    question: &str,
    correct_answers: Vec<&str>,
) -> Result<(), String> {
    let answers = correct_answers
        .iter()
        .map(|x| format!("{}\n", x))
        .fold("".to_string(), |acc, x| format!("{}{}", acc, x));

    match query(
        "INSERT INTO tasks
        (task_id, type, name, quest_id, desc, question, text_answers)
        VALUES
        (?,\'text\',?,?,?,?,?)",
    )
    .bind(task_id)
    .bind(name)
    .bind(quest_id)
    .bind(desc)
    .bind(question)
    .bind(answers)
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to add text task: {}", err)),
    }
}

pub async fn delete_task(db: &mut SqliteConnection, id: u32) -> Result<(), String> {
    match query("DELETE FROM tasks WHERE task_id = ?")
        .bind(id)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to delete task: {}", err)),
    }
}

pub async fn get_tasks(db: &mut SqliteConnection) -> Result<Vec<Task>, String> {
    let rows = match query("SELECT * FROM tasks").fetch_all(db).await {
        Ok(val) => val,
        Err(err) => return Err(format!("Failed to get tasks: {}", err)),
    };

    let tasks: Vec<Task> = rows
        .iter()
        .map(|row| {
            if let (
                Ok(Some(task_id)),
                Ok(Some(task_type)),
                Ok(Some(name)),
                Ok(quest_id),
                Ok(Some(desc)),
                Ok(Some(min_radius)),
                Ok(Some(max_radius)),
                Ok(location_to_duplicate),
            ) = (
                row.try_get("task_id"),
                row.try_get::<Option<&str>, _>("type"),
                row.try_get("name"),
                row.try_get("quest_id"),
                row.try_get("desc"),
                row.try_get("min_radius"),
                row.try_get("max_radius"),
                row.try_get("location_to_duplicate"),
            ) {
                if task_type != "location" {
                    return Task::Invalid("Task which matches the chracteristics of a location task is not marked as such".to_string());
                }

                return Task::Location(LocationTask {
                    task_id,
                    name,
                    quest_id,
                    desc,
                    min_radius,
                    max_radius,
                    location_to_duplicate,
                });
            }
            if let (
                Ok(Some(task_id)),
                Ok(Some(task_type)),
                Ok(Some(name)),
                Ok(quest_id),
                Ok(Some(desc)),
                Ok(Some(question)),
                Ok(Some(answers)),
                Ok(Some(choice_answers)),
            ) = (
                row.try_get("task_id"),
                row.try_get::<Option<&str>, _>("type"),
                row.try_get("name"),
                row.try_get("quest_id"),
                row.try_get("desc"),
                row.try_get("question"),
                row.try_get::<Option<&str>, _>("answers"),
                row.try_get::<Option<&str>, _>("choice_answers"),
            ) {
                if task_type != "choice" {
                    return Task::Invalid("Task which matches the chracteristics of a choice task is not marked as such".to_string());
                }

                let answers = answers.trim().split("\n").map(|x| x.to_owned()).collect();
                let choice_answers = choice_answers.trim().chars().enumerate().filter_map(|(i, x)| {
                    if x == '1' {
                        Some(i as u32)
                    } else {
                        None
                    }
                }).collect();

                return Task::Choice(ChoiceTask{
                    task_id,
                    name,
                    quest_id,
                    desc,
                    question,
                    answers,
                    choice_answers
                });
            }
            if let (
                Ok(Some(task_id)),
                Ok(Some(task_type)),
                Ok(Some(name)),
                Ok(quest_id),
                Ok(Some(desc)),
                Ok(Some(question)),
                Ok(Some(text_answers)),
            ) = (
                row.try_get("task_id"),
                row.try_get::<Option<&str>, _>("type"),
                row.try_get("name"),
                row.try_get("quest_id"),
                row.try_get("desc"),
                row.try_get("question"),
                row.try_get::<Option<&str>, _>("text_answers"),
            ) {
                if task_type != "text" {
                    return Task::Invalid("Task which matches the chracteristics of a text task is not marked as such".to_string());
                }
                
                let text_answers = text_answers.trim().split("\n").map(|x| x.to_owned()).collect();

                return Task::Text(TextTask{
                    task_id,
                    name,
                    quest_id,
                    desc,
                    question,
                    text_answers
                });
            }
            Task::Invalid("Task does not match any category".to_string())
        })
        .collect();

    Ok(tasks)
}

#[cfg(test)]
mod task_db_tests {
    use std::env;

    use rocket::local::asynchronous::Client;
    use rocket_db_pools::{Database, Pool};

    use crate::{rocket, DB};

    use super::{add_location_task, get_tasks, next_task_id, add_choice_task, add_text_task};

    #[rocket::async_test]
    async fn insert_and_get_back() {
        env::set_var("WIEDZIELISCIE_BACKEND_RESET_DB", "1");
        
        let client = Client::tracked(rocket())
            .await
            .expect("Failed to create client");

        let db = DB::fetch(client.rocket()).unwrap();

        let id = next_task_id(&mut db.get().await.unwrap()).await.unwrap();

        add_location_task(
            &mut db.get().await.unwrap(), 
            id,
            "test loc 1", 
            None, 
            "test desc 1", 
            1.0, 
            4.0, 
            None
        ).await.unwrap();
        
        add_choice_task(
            &mut db.get().await.unwrap(), 
            id,
            "test choice 1", 
            Some(1), 
            "test desc 2", 
            "R u dumb?",
            vec!["Yes", "No", "Sure"],
            vec![0, 2]
        ).await.unwrap();

        add_text_task(
            &mut db.get().await.unwrap(), 
            id,
            "test choice 1", 
            Some(1), 
            "test desc 2", 
            "R u dumb?",
            vec!["Yes", "No", "Sure"]
        ).await.unwrap();

        let tasks = get_tasks(&mut db.get().await.unwrap()).await.unwrap();
        println!("{:?}", tasks);

        assert_eq!(format!("{:?}", tasks), "[Location(LocationTask { task_id: 1, name: \"test loc 1\", quest_id: None, desc: \"test desc 1\", min_radius: 1.0, max_radius: 4.0, location_to_duplicate: None }), Choice(ChoiceTask { task_id: 1, name: \"test choice 1\", quest_id: Some(1), desc: \"test desc 2\", question: \"R u dumb?\", answers: [\"Yes\", \"No\", \"Sure\"], choice_answers: [0, 2] }), Text(TextTask { task_id: 1, name: \"test choice 1\", quest_id: Some(1), desc: \"test desc 2\", question: \"R u dumb?\", text_answers: [\"Yes\", \"No\", \"Sure\"] })]");
    }
}


//  ██████╗ ██╗   ██╗███████╗███████╗████████╗
// ██╔═══██╗██║   ██║██╔════╝██╔════╝╚══██╔══╝
// ██║   ██║██║   ██║█████╗  ███████╗   ██║   
// ██║▄▄ ██║██║   ██║██╔══╝  ╚════██║   ██║   
// ╚██████╔╝╚██████╔╝███████╗███████║   ██║   
//  ╚══▀▀═╝  ╚═════╝ ╚══════╝╚══════╝   ╚═╝   


#[derive(Debug, FromRow)]
pub struct QuestRow {
    pub quest_id: u32,
    pub desc: String,
    pub unlocks: String,
    pub points: u32,
    pub coins: u32,
    pub rewards: String
}

#[derive(Debug)]
pub struct Quest {
    pub quest_id: u32,
    pub desc: String,
    pub unlocks: Vec<u32>,
    pub points: u32,
    pub coins: u32,
    pub rewards: Vec<u32>
}

impl From<&QuestRow> for Quest {
    fn from(value: &QuestRow) -> Quest {
        
        let unlocks = value.unlocks.split('\n').map(|x| x.parse::<u32>().unwrap_or(0)).collect();
        let rewards = value.rewards.split('\n').map(|x| x.parse::<u32>().unwrap_or(0)).collect();

        Quest {
            quest_id: value.quest_id,
            desc: value.desc.clone(),
            coins: value.coins,
            points: value.points,
            rewards,
            unlocks
        }
    }
}

pub async fn next_quest_id(db: &mut SqliteConnection) -> Result<u32, String> {
    match query("SELECT MAX(quest_id) FROM quests")
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

pub async fn create_quest(
    db: &mut SqliteConnection,
    id: u32,
    desc: &str,
    unlocks: Vec<u32>,
    points: u32,
    coins: u32,
    rewards: Vec<u32>,
) -> Result<(), String> {
    let unlocks_str: String = unlocks.iter().map(|x| {let mut y = x.to_string(); y.push('\n'); y})
        .fold("".to_string(), |mut acc, x| { acc.push_str(&x); acc });
    
    let rewards_str: String = rewards.iter().map(|x| {let mut y = x.to_string(); y.push('\n'); y})
        .fold("".to_string(), |mut acc, x| { acc.push_str(&x); acc });

    match query(
        "INSERT INTO
        quests (quest_id, desc, unlocks, points, coins, rewards)
        VALUES (?,?,?,?,?,?)",
    )
    .bind(id)
    .bind(desc)
    .bind(unlocks_str)
    .bind(points)
    .bind(coins)
    .bind(rewards_str)
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to create quest: {}", err)),
    }
}

pub async fn delete_quest(db: &mut SqliteConnection, id: u32) -> Result<(), String> {
    match query("DELETE FROM quests WHERE quest_id = ?")
        .bind(id)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to delete quest: {}", err)),
    }
}

pub async fn get_all_quests(db: &mut SqliteConnection) -> Result<Vec<Quest>, String> {
    let rows = match query_as::<_, QuestRow>("SELECT * FROM quests")
        .fetch_all(db)
        .await
    {
        Ok(val) => val,
        Err(err) => return Err(format!("Failed to get quests: {}", err)),
    };

    Ok(rows.iter().map(|row| {
        Quest::from(row)
    }).collect())
}

//  ██████╗ ██╗   ██╗███████╗███████╗████████╗    ███████╗████████╗ █████╗  ██████╗ ███████╗
// ██╔═══██╗██║   ██║██╔════╝██╔════╝╚══██╔══╝    ██╔════╝╚══██╔══╝██╔══██╗██╔════╝ ██╔════╝
// ██║   ██║██║   ██║█████╗  ███████╗   ██║       ███████╗   ██║   ███████║██║  ███╗█████╗  
// ██║▄▄ ██║██║   ██║██╔══╝  ╚════██║   ██║       ╚════██║   ██║   ██╔══██║██║   ██║██╔══╝  
// ╚██████╔╝╚██████╔╝███████╗███████║   ██║       ███████║   ██║   ██║  ██║╚██████╔╝███████╗
//  ╚══▀▀═╝  ╚═════╝ ╚══════╝╚══════╝   ╚═╝       ╚══════╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚══════╝

#[derive(Debug, FromRow)]
struct QuestStageRow {
    _quest_id: u32,
    stage_id: u32,
    task_id: Option<u32>,
    dialogue_id: Option<u32>,
    task_name: Option<String>,
    task_type: Option<String>,
    dialogue_name: Option<String>
}

#[derive(Debug)]
pub struct QuestStage {
    pub stage_id: u32,
    pub content_id: u32,
    pub stage_type: String,
    pub name: String
}

#[derive(Debug)]
pub enum QuestStageContent {
    Dialogue(u32),
    Task(u32)
}

pub async fn next_quest_stage_id(db: &mut SqliteConnection, quest_id: u32) -> Result<u32, String> {
    match query("SELECT MAX(stage_id) FROM quest_stages WHERE quest_id = ?")
        .bind(quest_id)
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

pub async fn add_quest_stage(db: &mut SqliteConnection, quest_id: u32, stage_id: u32, content: QuestStageContent) -> Result<(), String> {
    let task_id: Option<u32> = match content {
        QuestStageContent::Task(val) => Some(val),
        QuestStageContent::Dialogue(_) => None
    };
    
    let dialogue_id: Option<u32> = match content {
        QuestStageContent::Dialogue(val) => Some(val),
        QuestStageContent::Task(_) => None
    };
    
    match query(
        "INSERT INTO
        quest_stages
        (quest_id, stage_id, task_id, dialogue_id)
        VALUES (?,?,?,?)",
    )
    .bind(quest_id)
    .bind(stage_id)
    .bind(task_id)
    .bind(dialogue_id)
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to add quest stage: {}", err)),
    }
}

pub async fn get_all_quest_stages(db: &mut SqliteConnection, quest_id: u32) -> Result<Vec<QuestStage>, String> {
    let rows = match query_as::<_, QuestStageRow>("SELECT quest_stages.quest_id, quest_stages.stage_id, quest_stages.task_id, quest_stages.dialogue_id, tasks.name, tasks.type, dialogues.name
        JOIN tasks ON tasks.task_id = quest_stages.task_id
        JOIN dialogues ON dialogues.dialogue_id = quest_stages.dialogue_id
        WHERE quest_stages.quest_id = ?")
        .bind(quest_id)
        .fetch_all(db)
        .await 
        {
            Ok(val) => val,
            Err(err) => return Err(format!("Failed to get quest stages: {}", err))
        };

    Ok(rows.iter().filter_map(|row| {
        if let (Some(dialogue_id), Some(dialogue_name)) = (row.dialogue_id, &row.dialogue_name) {
            Some(QuestStage {
                stage_id: row.stage_id,
                name: dialogue_name.clone(),
                content_id: dialogue_id,
                stage_type: String::from("dialogue")
            })
        } else if let (Some(task_id), Some(task_name), Some(task_type)) = (row.task_id, &row.task_name, &row.task_type) {
            Some(QuestStage {
                stage_id: row.stage_id,
                name: task_name.clone(),
                content_id: task_id,
                stage_type: task_type.clone()
            })
        } else {
            None
        }
    }).collect())
}

pub async fn delete_quest_stage(db: &mut SqliteConnection, quest_id: u32, stage_id: u32) -> Result<(), String> {
    match query("DELETE FROM quest_stages WHERE quest_id = ? AND stage_id = ?")
        .bind(quest_id)
        .bind(stage_id)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to delete quest stage: {}", err)),
    }
}

pub async fn change_quest_stage_id(db: &mut SqliteConnection, quest_id: u32, from: u32, to: u32, delta: i32) -> Result<(), String> {
    match query("UPDATE quest_stages 
        SET stage_id = stage_id + ? 
        WHERE quest_id = ? AND stage_id >= ? AND stage_id <= ?")
        .bind(delta)
        .bind(quest_id)
        .bind(from)
        .bind(to)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to change quest stage ids: {}", err))
    }
}
