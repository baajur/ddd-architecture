use super::entity::{Id, Post};
use async_trait::async_trait;

pub mod errors {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Find {
        #[error("Post not found")]
        NotFound,

        #[error(transparent)]
        Other(#[from] anyhow::Error),
    }

    #[derive(Error, Debug)]
    pub enum Save {
        #[error(transparent)]
        Other(#[from] anyhow::Error),
    }
}

#[async_trait]
pub trait RepositoryInterface: Send + Sync {
    async fn find(&self, id: &Id) -> Result<Post, errors::Find>;
    async fn save(&self, user: &Post) -> Result<(), errors::Save>;
}
