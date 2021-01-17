use super::entity::{Id, User};
use async_trait::async_trait;

pub mod errors {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Find {
        #[error("User not found")]
        NotFound,

        #[error(transparent)]
        Other(#[from] anyhow::Error),
    }

    #[derive(Error, Debug)]
    pub enum Save {
        #[error("Nickname \"{nickname}\" already exists")]
        NicknameExists { nickname: String },

        #[error(transparent)]
        Other(#[from] anyhow::Error),
    }
}

#[async_trait]
pub trait RepositoryInterface: Send + Sync {
    async fn find(&self, id: &Id) -> Result<User, errors::Find>;
    async fn find_by_nickname(&self, nickname: &str) -> Result<User, errors::Find>;
    async fn save(&self, user: &User) -> Result<(), errors::Save>;
}
