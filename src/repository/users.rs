use serde::Serialize;
use sqlx::{Error, FromRow, PgPool};
use utoipa::ToSchema;

use crate::repository::Table;

#[derive(Clone)] 
pub struct UsersRepo {
    pool: PgPool
}
impl UsersRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn find_by_key(&self, id: i64) -> Result<Users, Error> {
        sqlx::query_as::<_, Users>(
            "SELECT id, name FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }
    pub async fn insert(&self, name: String) -> Result<(), Error> {
        sqlx::query_as!(
            Users,
            "INSERT INTO users (name) VALUES ($1)",
            name
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[derive(FromRow, ToSchema, Serialize, Default)]
pub struct Users {
    pub id: i64,
    pub name: String
}
impl Table for Users {}