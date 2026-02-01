pub mod users;
pub mod post;
pub mod comment;

use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;

pub trait Repo {
    fn new(pool: PgPool) -> Self;
}
pub trait Table: ToSchema + Serialize {

}

#[derive(Clone)]
pub struct RepoFactory {
    pub user: users::UsersRepo,
    // pub post: post::PostRepo,
    // pub comment: comment::CommentRepo
}
impl RepoFactory {
    pub fn new(pool: PgPool) -> Self {
        Self {
            user: users::UsersRepo::new(pool.clone()),
            // post: post::PostRepo::new(pool.clone()),
            // comment: comment::CommentRepo::new(pool.clone())
        }
    }
}