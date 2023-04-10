use crate::crud::connection::establish_connection;
use sqlx::types::uuid::Uuid;

#[derive(Debug, Default)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

impl User {
    
}

pub async fn insert_user(id: Uuid, username: String, password: String) -> Result<(), sqlx::Error> {
    let pool = establish_connection().await?;

    sqlx::query!(
        "INSERT INTO users (id, username, password) VALUES ($1, $2, $3)",
        id,
        username,
        password
    )
    .execute(&pool)
    .await
    .unwrap();


    Ok(())
}

pub async fn get_user(username: String, password: String) -> Result<Vec<User>, sqlx::Error> {
    let pool = establish_connection().await?;
    let users = sqlx::query_as!(
        User,
        "select * from users where username = $1 and password = $2",
        username,
        password
    ).fetch_all(&pool).await.unwrap();

    Ok(users)
}
