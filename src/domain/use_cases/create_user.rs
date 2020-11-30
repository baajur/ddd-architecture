use async_trait::async_trait;
use shaku::Provider;

use super::super::user::{RepositoryInterface, ServiceInterface, User};

#[async_trait]
pub trait Interface {
    async fn call(&self, name: &str) -> Result<User, ()>;
}

// Could UseCase be also the Interface ? possible to remove interface or set interface = UseCase?
#[derive(Provider)]
#[shaku(interface = Interface)]
pub struct UseCase {
    #[shaku(provide)]
    repository: Box<dyn RepositoryInterface + Send + Sync>,
    #[shaku(provide)]
    service: Box<dyn ServiceInterface + Send + Sync>,
}

#[async_trait]
impl Interface for UseCase {
    async fn call(&self, name: &str) -> Result<User, ()> {
        let user = User::new(name);
        self.repository.save(&user).await?;
        Ok(user)
    }
}

/*
#[cfg(test)]
mod test {
    use super::super::super::user::{RepositoryInterface, ServiceInterface, User};
    use super::{Interface, UseCase};

    struct FakeRepository;

    impl RepositoryInterface for FakeRepository {
        fn save(&self, _user: &User) -> Result<(), ()> {
            println!("Call the fake save...");
            Ok(())
        }
    }

    struct FakeService;

    impl ServiceInterface for FakeService {
        fn get_double(&self) -> usize {
            3
        }
    }

    #[test]
    fn it_creates_a_user() {
        let use_case = UseCase {
            repository: Box::new(FakeRepository {}),
            service: Box::new(FakeService {}),
        };

        let user = use_case.call("toto");
        assert_eq!(user, Ok(User::new("toto")));
    }
}
*/
