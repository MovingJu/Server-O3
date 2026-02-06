use serde::Serialize;
use sqlx::{Error, FromRow, PgPool};
use utoipa::ToSchema;

use super::{Repo, Table};

#[derive(Clone)]
pub struct UsersRepo {
    pool: PgPool,
}
#[async_trait::async_trait]
impl Repo<Users> for UsersRepo {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    async fn insert(&self, row: &Users) -> Result<(), Error> {
        sqlx::query_as!(Users, "INSERT INTO users (name) VALUES ($1)", row.name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn select(&self, criteria: &Users) -> Result<Vec<Users>, Error> {
        let res = sqlx::query_as::<_, Users>("SELECT id, name FROM users WHERE name = $1")
            .bind(&criteria.name)
            .fetch_one(&self.pool)
            .await?;
        Ok(vec![res])
    }
}

#[derive(FromRow, ToSchema, Serialize, Default)]
pub struct Users {
    pub id: i64,
    pub name: String,
}
impl Table for Users {}
