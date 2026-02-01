//! # sql_pool
//! Sql management module
use axum::extract::FromRef;
use sqlx::PgPool;

/// # AppState
/// PgPool은 내부적으로 Arc이므로 Clone 비용이 낮다.
#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(input: &AppState) -> Self {
        input.pool.clone()
    }
}

