use async_trait::async_trait;
use shaku::Provider;

use super::super::post::{errors, Post, RepositoryInterface};

#[async_trait]
pub trait Interface: Send + Sync {
    async fn call(&self, content: &str) -> Result<Post, errors::Save>;
}

#[derive(Provider)]
#[shaku(interface = Interface)]
pub struct UseCase {
    #[shaku(provide)]
    repository: Box<dyn RepositoryInterface + Send + Sync>,
}

impl std::fmt::Debug for UseCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreatePostUseCase").finish()
    }
}

#[async_trait]
impl Interface for UseCase {
    #[tracing::instrument(name = "Create a post")]
    async fn call(&self, content: &str) -> Result<Post, errors::Save> {
        let post = Post::new(content);
        self.repository.save(&post).await?;
        Ok(post)
    }
}

#[cfg(test)]
mod test {
    use super::super::super::post::{errors, Id, Post, RepositoryInterface};
    use super::{Interface, UseCase};
    use async_trait::async_trait;

    struct FakeRepository;

    #[async_trait]
    impl RepositoryInterface for FakeRepository {
        async fn find(&self, _id: &Id) -> Result<Post, errors::Find> {
            Ok(Post::new("todo"))
        }

        async fn save(&self, _post: &Post) -> Result<(), errors::Save> {
            println!("Call the fake save...");
            Ok(())
        }
    }

    #[tokio::test]
    async fn it_creates_a_post() {
        let use_case = UseCase {
            repository: Box::new(FakeRepository {}),
        };

        // Fake id call
        let result = use_case.call("My new post").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().content(), "My new post");
    }
}
