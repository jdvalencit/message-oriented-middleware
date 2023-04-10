use sqlx::postgres::{PgPoolOptions};
use sqlx::{Pool, Postgres};
use std::env::var;

pub async fn establish_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let db_url = var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
    // let db_url = "postgres://postgres:secret@localhost:5432/momdb".to_string();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    Ok(pool)
}
