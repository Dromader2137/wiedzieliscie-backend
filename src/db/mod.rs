use sqlx::{pool::PoolConnection, query, Sqlite, SqliteConnection};

async fn create_user_table<'a>(db: &mut SqliteConnection) -> Result<(), String> {
    match query("CREATE TABLE users (
        user_id int,
        first_name varchar(255),
        last_name varchar(255),
        email varchar(255),
        password varchar(255),
        gender bool
    )")
        .execute(db).await {
        Err(err) => Err(format!("Failed to create users table: {}", err)),
        _ => Ok(())
    }
}

pub async fn create_tables(mut db: PoolConnection<Sqlite>) {
    match create_user_table(&mut db).await {
        Err(err) => { panic!("{}", err) },
        _ => {}
    }
}
