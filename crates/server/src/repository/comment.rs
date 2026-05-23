use schemars::JsonSchema;
use serde::Serialize;
use sqlx::{Error, FromRow, PgPool};

#[derive(Clone)]
pub struct CommentRepo {
    pool: PgPool,
}

#[derive(FromRow, Serialize, JsonSchema)]
pub struct Comment {
    pub id: i64,
    pub username: String,
    pub content: String,
    pub created_at: String,
}

impl CommentRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_by_post(&self, post_id: &str) -> Result<Vec<Comment>, Error> {
        sqlx::query_as::<_, Comment>(
            "SELECT id, username, content, created_at::text \
             FROM comments WHERE post_id = $1 ORDER BY created_at ASC",
        )
        .bind(post_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn insert(&self, post_id: &str, username: &str, content: &str) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO comments (post_id, username, content) VALUES ($1, $2, $3)",
        )
        .bind(post_id)
        .bind(username)
        .bind(content)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
