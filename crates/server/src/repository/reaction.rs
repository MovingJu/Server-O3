use sqlx::{Error, FromRow, PgPool};
use std::collections::HashMap;

#[derive(Clone)]
pub struct ReactionRepo {
    pool: PgPool,
}

#[derive(FromRow)]
struct ReactionRow {
    reaction: String,
    count: i64,
}

impl ReactionRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_counts(&self, post_id: &str) -> Result<HashMap<String, i64>, Error> {
        let rows = sqlx::query_as::<_, ReactionRow>(
            "SELECT reaction, COUNT(*)::bigint AS count FROM reactions WHERE post_id = $1 GROUP BY reaction",
        )
        .bind(post_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(|r| (r.reaction, r.count)).collect())
    }

    pub async fn get_user_reactions(&self, post_id: &str, username: &str) -> Result<Vec<String>, Error> {
        let rows = sqlx::query_as::<_, (String,)>(
            "SELECT reaction FROM reactions WHERE post_id = $1 AND username = $2",
        )
        .bind(post_id)
        .bind(username)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.0).collect())
    }

    /// Returns true if reaction was added, false if removed.
    pub async fn toggle(&self, post_id: &str, username: &str, reaction: &str) -> Result<bool, Error> {
        let deleted = sqlx::query(
            "DELETE FROM reactions WHERE post_id = $1 AND username = $2 AND reaction = $3",
        )
        .bind(post_id)
        .bind(username)
        .bind(reaction)
        .execute(&self.pool)
        .await?
        .rows_affected();

        if deleted == 0 {
            sqlx::query(
                "INSERT INTO reactions (post_id, username, reaction) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            )
            .bind(post_id)
            .bind(username)
            .bind(reaction)
            .execute(&self.pool)
            .await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
