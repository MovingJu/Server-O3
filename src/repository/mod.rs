mod user;
mod post;
mod comment;

use sqlx::PgPool;

pub trait Repo {
    fn new(pool: PgPool) -> Self;
}
pub trait Table: IntoIterator {

}

#[derive(Clone)]
pub struct RepoFactory {
    pub user: user::UserRepo,
    // pub post: post::PostRepo,
    // pub comment: comment::CommentRepo
}
impl RepoFactory {
    pub fn new(pool: PgPool) -> Self {
        Self {
            user: user::UserRepo::new(pool.clone()),
            // post: post::PostRepo::new(pool.clone()),
            // comment: comment::CommentRepo::new(pool.clone())
        }
    }
}