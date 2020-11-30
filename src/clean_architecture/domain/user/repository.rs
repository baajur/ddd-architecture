use super::model::User;
use shaku::Provider;

// Repository
pub trait RepositoryInterface {
    fn save(&self, user: &User) -> Result<(), ()>;
}

#[derive(Provider)]
#[shaku(interface = RepositoryInterface)]
pub struct Repository {}

impl RepositoryInterface for Repository {
    fn save(&self, user: &User) -> Result<(), ()> {
        println!("Repository is saving: {:#?}", user);
        Ok(())
    }
}
