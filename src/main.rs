use rocket_db_pools::Database;

#[macro_use] extern crate rocket;

pub mod user;

#[derive(Database)]
#[database("db")]
pub struct DB(sqlx::SqlitePool);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
