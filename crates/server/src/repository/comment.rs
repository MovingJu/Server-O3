use sqlx::PgPool;

#[derive(Clone)]
pub struct CommentRepo {
    pool: PgPool,
}
impl CommentRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
