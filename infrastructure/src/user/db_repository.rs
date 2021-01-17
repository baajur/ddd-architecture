use crate::libs::db::service::DBServiceInterface;
use async_trait::async_trait;
use domain::{
    libs::id::Identifier,
    user::{errors, Id, RepositoryInterface, User},
};
use shaku::Provider;
use sqlx::{error::Error as sqlxError, postgres::PgDatabaseError, types::Uuid};

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
        f.debug_struct("UserDbRepository").finish()
    }
}

#[async_trait]
impl RepositoryInterface for DbRepository {
    #[tracing::instrument]
    async fn find(&self, id: &Id) -> Result<User, errors::Find> {
        let executor = &self
            .db_service
            .executor()
            .await
            .map_err(|err| errors::Find::Other(err.into()))?;

        sqlx::query!("SELECT * FROM users WHERE id = $1", to_pg_uuid(id))
            .fetch_one(executor)
            .await
            .map_err(|err: sqlxError| match err {
                sqlxError::RowNotFound => errors::Find::NotFound,
                _ => errors::Find::Other(err.into()),
            })
            .map(|record| User::build_from_repository(from_pg_uuid(record.id), record.nickname))
    }

    #[tracing::instrument]
    async fn find_by_nickname(&self, nickname: &str) -> Result<User, errors::Find> {
        let executor = &self
            .db_service
            .executor()
            .await
            .map_err(|err| errors::Find::Other(err.into()))?;

        sqlx::query!("SELECT * FROM users WHERE nickname = $1", nickname)
            .fetch_one(executor)
            .await
            .map_err(|err: sqlxError| match err {
                sqlxError::RowNotFound => errors::Find::NotFound,
                _ => errors::Find::Other(err.into()),
            })
            .map(|record| User::build_from_repository(from_pg_uuid(record.id), record.nickname))
    }

    #[tracing::instrument]
    async fn save(&self, user: &User) -> Result<(), errors::Save> {
        let executor = &self
            .db_service
            .executor()
            .await
            .map_err(|err| errors::Save::Other(err.into()))?;

        sqlx::query!(
            "INSERT INTO users (id, nickname) VALUES ($1, $2)",
            to_pg_uuid(user.id()),
            user.nickname()
        )
        .execute(executor)
        .await
        .map_err(|err: sqlxError| match err {
            sqlxError::Database(db_err)
                if Some("users_nickname_unique")
                    == db_err.downcast_ref::<PgDatabaseError>().constraint() =>
            {
                errors::Save::NicknameExists {
                    nickname: user.nickname().into(),
                }
            }
            err => errors::Save::Other(err.into()),
        })
        .and(Ok(()))
    }
}
