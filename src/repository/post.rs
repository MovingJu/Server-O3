use sqlx::PgPool;

#[derive(Clone)]
pub struct PostRepo {
    pool: PgPool
}
impl PostRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}