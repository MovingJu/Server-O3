use sqlx::{Error, FromRow, PgPool};

#[derive(Clone)]
pub struct AuthRepo {
    pool: PgPool,
}

#[derive(FromRow)]
pub struct AuthUser {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl AuthRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO auth_users (username, email, password_hash) VALUES ($1, $2, $3)",
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<AuthUser>, Error> {
        sqlx::query_as::<_, AuthUser>(
            "SELECT id, username, email, password_hash FROM auth_users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
    }
}
