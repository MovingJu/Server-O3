use sqlx::{PgPool, FromRow};

#[derive(Clone)] 
pub struct UserRepo {
    pool: PgPool
}
impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(FromRow)]
pub struct User {
    pub id: usize,
    pub name: String
}