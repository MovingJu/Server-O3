pub mod posts;
pub mod users;
// pub mod comment;

use schemars::JsonSchema;
use serde::Serialize;
use sqlx::{Error, PgPool};

#[async_trait::async_trait]
pub trait Repo<T: Table> {
    fn new(pool: PgPool) -> Self;
    async fn select(&self, criteria: &T) -> Result<Vec<T>, Error>;
    async fn insert(&self, row: &T) -> Result<(), Error>;
}
pub trait Table: JsonSchema + Serialize + Sized {}

#[derive(Clone)]
pub struct RepoFactory {
    pub user: users::UsersRepo,
    pub posts: posts::PostsRepo,
    // pub comment: comment::CommentRepo
}
impl RepoFactory {
    pub fn new(pool: PgPool) -> Self {
        Self {
            user: users::UsersRepo::new(pool.clone()),
            posts: posts::PostsRepo::new(pool.clone()),
            // comment: comment::CommentRepo::new(pool.clone())
        }
    }
}
