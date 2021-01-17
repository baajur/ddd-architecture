use async_trait::async_trait;
use shaku::Provider;

use super::super::user::{errors, RepositoryInterface, User};

#[async_trait]
pub trait Interface: Send + Sync {
    async fn call(&self, nickname: &str) -> Result<User, errors::Save>;
}

#[derive(Provider)]
#[shaku(interface = Interface)]
pub struct UseCase {
    #[shaku(provide)]
    repository: Box<dyn RepositoryInterface + Send + Sync>,
}

impl std::fmt::Debug for UseCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateUserUseCase").finish()
    }
}

#[async_trait]
impl Interface for UseCase {
    #[tracing::instrument(name = "Create a user")]
    async fn call(&self, nickname: &str) -> Result<User, errors::Save> {
        let user = User::new(nickname);
        self.repository.save(&user).await?;
        Ok(user)
    }
}

#[cfg(test)]
mod test {
    use super::super::super::user::{errors, Id, RepositoryInterface, User};
    use super::{Interface, UseCase};
    use async_trait::async_trait;

    struct FakeRepository;

    #[async_trait]
    impl RepositoryInterface for FakeRepository {
        async fn find(&self, _id: &Id) -> Result<User, errors::Find> {
            Ok(User::new("todo"))
        }

        async fn find_by_nickname(&self, nickname: &str) -> Result<User, errors::Find> {
            Ok(User::new(nickname)) // TODO
        }

        async fn save(&self, _user: &User) -> Result<(), errors::Save> {
            println!("Call the fake save...");
            Ok(())
        }
    }

    #[tokio::test]
    async fn it_creates_a_user() {
        let use_case = UseCase {
            repository: Box::new(FakeRepository {}),
        };

        // Fake id call
        let result = use_case.call("Elliot").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().nickname(), "Elliot");
    }
}
