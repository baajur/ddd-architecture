use crate::libs::db::service::DBServiceInterface;
use async_trait::async_trait;
use domain::{
    libs::id::Identifier,
    post::{errors, Id, Post, RepositoryInterface},
};
use shaku::Provider;
use sqlx::{error::Error as sqlxError, types::Uuid};

fn to_pg_uuid(id: &Id) -> Uuid {
    Uuid::from_u128(id.inner_value())
}

fn from_pg_uuid(pg_uuid: Uuid) -> Id {
    Id::new(pg_uuid.as_u128())
}

#[derive(Provider)]
#[shaku(interface = RepositoryInterface + Send + Sync)]
pub struct DbRepository {
    #[shaku(provide)]
    db_service: Box<dyn DBServiceInterface + Send + Sync>,
}

impl std::fmt::Debug for DbRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PostDbRepository").finish()
    }
}

#[async_trait]
impl RepositoryInterface for DbRepository {
    #[tracing::instrument]
    async fn find(&self, id: &Id) -> Result<Post, errors::Find> {
        let executor = &self
            .db_service
            .executor()
            .await
            .map_err(|err| errors::Find::Other(err.into()))?;

        sqlx::query!("SELECT * FROM posts WHERE id = $1", to_pg_uuid(id))
            .fetch_one(executor)
            .await
            .map_err(|err: sqlxError| match err {
                sqlxError::RowNotFound => errors::Find::NotFound,
                _ => errors::Find::Other(err.into()),
            })
            .map(|record| Post::build_from_repository(from_pg_uuid(record.id), record.content))
    }

    #[tracing::instrument]
    async fn save(&self, post: &Post) -> Result<(), errors::Save> {
        let executor = &self
            .db_service
            .executor()
            .await
            .map_err(|err| errors::Save::Other(err.into()))?;

        sqlx::query!(
            "INSERT INTO posts (id, content) VALUES ($1, $2)",
            to_pg_uuid(post.id()),
            post.content()
        )
        .execute(executor)
        .await
        .map_err(|err| errors::Save::Other(err.into()))
        .and(Ok(()))
    }
}
