use schemars::JsonSchema;
use serde::Serialize;
use sqlx::{Error, FromRow, PgPool};

use super::{Repo, Table};

#[derive(Clone)]
pub struct PostsRepo {
    pool: PgPool,
}
impl PostsRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
#[async_trait::async_trait]
impl Repo<Posts> for PostsRepo {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    async fn select(&self, criteria: &Posts) -> Result<Vec<Posts>, Error> {
        let query = "SELECT id, title, content, user_id FROM posts WHERE name = $1".to_string();
        let res = sqlx::query_as::<_, Posts>(&query)
            .bind(&criteria.user_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(vec![res])
    }
    async fn insert(&self, _row: &Posts) -> Result<(), Error> {
        todo!();
    }
}

#[derive(FromRow, Serialize, JsonSchema, Default)]
pub struct Posts {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub user_id: i64,
}
impl Table for Posts {}
