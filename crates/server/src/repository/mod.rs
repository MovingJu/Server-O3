#![allow(dead_code)]

pub mod auth;
pub mod comment;
pub mod posts;
pub mod reaction;
pub mod users;

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
    pub auth: auth::AuthRepo,
    pub comment: comment::CommentRepo,
    pub reaction: reaction::ReactionRepo,
    pub user: users::UsersRepo,
    pub posts: posts::PostsRepo,
}
impl RepoFactory {
    pub fn new(pool: PgPool) -> Self {
        Self {
            auth: auth::AuthRepo::new(pool.clone()),
            comment: comment::CommentRepo::new(pool.clone()),
            reaction: reaction::ReactionRepo::new(pool.clone()),
            user: users::UsersRepo::new(pool.clone()),
            posts: posts::PostsRepo::new(pool.clone()),
        }
    }
}
