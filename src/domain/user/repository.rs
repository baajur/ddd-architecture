use super::model::User;
use async_trait::async_trait;

#[async_trait]
pub trait RepositoryInterface {
    async fn save(&self, user: &User) -> Result<(), ()>;
}
