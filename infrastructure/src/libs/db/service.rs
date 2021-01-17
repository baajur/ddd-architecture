use async_trait::async_trait;
use dotenv::dotenv;
use shaku::Provider;
use sqlx::{
    error::Error,
    pool::Pool,
    postgres::{PgPool, Postgres},
};
use std::env;

#[async_trait]
pub trait DBServiceInterface {
    async fn executor(&self) -> Result<Pool<Postgres>, Error>;
}

#[derive(Provider)]
#[shaku(interface = DBServiceInterface + Send + Sync)]
pub struct DBService {}

#[async_trait]
impl DBServiceInterface for DBService {
    async fn executor(&self) -> Result<Pool<Postgres>, Error> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgPool::connect(&database_url).await
    }
}
