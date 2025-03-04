use db::create_tables;
use rocket::fairing::AdHoc;
use rocket_db_pools::{Database, Pool};

#[macro_use]
extern crate rocket;

pub mod admin;
pub mod db;
pub mod error;
pub mod fetch;
pub mod user;
pub mod util;

#[derive(Database)]
#[database("db")]
pub struct DB(sqlx::SqlitePool);

#[launch]
fn rocket() -> _ {
    util::load_env().unwrap();
    let db = DB::init();
    rocket::build()
        .attach(db)
        .attach(AdHoc::on_liftoff("Startup Check", |rocket| {
            Box::pin(async move {
                let DB(db) = DB::fetch(rocket).expect("Failed to init the database");
                let connection = db.get().await.expect("Failed to init the database");
                create_tables(connection).await;
            })
        }))
        .mount(
            "/",
            routes![
                user::register::auth_register,
                user::register::auth_resend_verification,
                user::register::auth_verify,
                user::login::auth_login,
                user::reset::auth_password_reset,
                user::reset::auth_password_reset_verify,
                user::logout::auth_logout,
                user::retrieve::auth_retrieve_user,
                user::verifyless_updates::user_modify_first_name,
                user::verifyless_updates::user_modify_last_name,
                user::verifyless_updates::user_modify_gender,
                user::verifyless_updates::user_modify_password,
                user::update_email::user_modify_email,
                user::update_email::user_modify_email_verify,
                user::delete_user::delete_user,
                admin::character::admin_characters_add,
                admin::character::admin_characters_delete,
                admin::character::admin_characters_get,
                admin::dialogue::admin_dialogues_add,
                admin::dialogue::admin_dialogues_delete,
                admin::dialogue::admin_dialogues_get,
                admin::dialogue::admin_dialogues_get_unused,
                admin::task::admin_tasks_location_add,
                admin::task::admin_tasks_multiple_choice_add,
                admin::task::admin_tasks_text_answer_add,
                admin::task::admin_tasks_get,
                admin::task::admin_tasks_get_unused,
                admin::quest::admin_quests_add,
                admin::quest::admin_quests_delete,
                admin::quest::admin_quests_get,
                admin::quest::admin_quests_duplicate,
                admin::quest::admin_quests_stages_add,
                admin::quest::admin_quests_stages_delete,
                admin::quest::admin_quests_stages_get,
                admin::quest::admin_quests_stages_move_back,
                admin::quest::admin_quests_stages_move_forward,
                admin::game::admin_game_pause,
                admin::game::admin_game_unpause,
                admin::game::admin_game_set_location_radius,
                admin::game::admin_quests_select_tutorial,
                error::report::report_error,
                error::report::report_suggestion,
                error::report::admin_get_reports,
                error::report::admin_get_logs,
                fetch::get::get_location_radius,
                fetch::get::get_pause_state,
                fetch::get::get_character,
                user::retrieve::user_retrieve_email,
                user::retrieve::user_retrieve_id,
                user::retrieve::user_retrieve_name,
                user::retrieve::user_retrieve_count,
            ],
        )
}
