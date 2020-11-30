use async_trait::async_trait;
use shaku::Provider;

use super::super::super::domain::user::{RepositoryInterface, User};

#[derive(Provider)]
#[shaku(interface = RepositoryInterface + Send + Sync)]
pub struct DbRepository {}

#[async_trait]
impl RepositoryInterface for DbRepository {
    async fn save(&self, user: &User) -> Result<(), ()> {
        println!("Repository is saving: {:#?}", user);
        Ok(())
    }
}
