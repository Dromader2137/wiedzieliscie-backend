use db::create_tables;
use rocket::fairing::AdHoc;
use rocket_db_pools::{Database, Pool};

#[macro_use]
extern crate rocket;

pub mod db;
pub mod user;

#[derive(Database)]
#[database("db")]
pub struct DB(sqlx::SqlitePool);

#[launch]
fn rocket() -> _ {
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
        .mount("/", routes![
            user::register::auth_register,
            user::register::auth_resend_verification,
            user::register::auth_verify,
            user::login::auth_login,
            user::reset::auth_password_reset,
            user::reset::auth_password_reset_verify,
        ])
}
