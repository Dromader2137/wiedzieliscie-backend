use rocket_db_pools::{Connection, Database};

#[macro_use] extern crate rocket;

pub mod user;

#[derive(Database)]
#[database("db")]
pub struct DB(sqlx::SqlitePool);

pub async fn create_tables(db: Connection<DB>) {
    let mut db = db.into_inner();

    

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![user::register::auth_register])
}
