use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_actix_web::{Request, Response, WSSubscription};
use domain::{
    post::Post,
    use_cases::create_post::{Interface as CreatePostUseCase, UseCase as CreatePostUseCaseImpl},
    use_cases::create_user::{Interface as CreateUserUseCase, UseCase as CreateUserUseCaseImpl},
    user::RepositoryInterface as UserRepositoryInterface,
    user::{Id as UserId, User},
};
use infrastructure::{
    libs::db::service::DBService, post::db_repository::DbRepository as PostDbRepository,
    user::db_repository::DbRepository as UserDbRepository,
};
use shaku::{module, HasProvider};
use telemetry::{get_subscriber, init_subscriber};
use tracing_actix_web::TracingLogger;

mod telemetry;

async fn index(schema: web::Data<GqlSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

async fn index_ws(
    schema: web::Data<GqlSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    WSSubscription::start(Schema::clone(&*schema), &req, payload)
}

// Module
module! {
    AppModule {
        components = [],
        providers = [
            UserDbRepository,
            PostDbRepository,
            CreateUserUseCaseImpl,
            CreatePostUseCaseImpl,
            DBService,
        ]
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("web".into(), "info".into());
    init_subscriber(subscriber);

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(AppModule::builder().build())
        .finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

use async_graphql::{Context, Object, Subscription, ID};
use futures::{Stream, StreamExt};
use std::time::Duration;

pub type GqlSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

#[derive(Clone)]
pub struct GraphUser {
    id: ID,
    nickname: String,
}

#[Object]
impl GraphUser {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn nickname(&self) -> &str {
        &self.nickname
    }
}

impl GraphUser {
    fn build(user: User) -> GraphUser {
        GraphUser {
            id: ID(user.id().to_string()),
            nickname: user.nickname().to_owned(),
        }
    }
}

#[derive(Clone)]
pub struct GraphPost {
    id: ID,
    content: String,
}

#[Object]
impl GraphPost {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn content(&self) -> &str {
        &self.content
    }
}

impl GraphPost {
    fn build(post: Post) -> GraphPost {
        GraphPost {
            id: ID(post.id().to_string()),
            content: post.content().to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    #[tracing::instrument(skip(ctx))]
    async fn user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Get a user")] id: ID,
    ) -> async_graphql::Result<Option<GraphUser>> {
        let user_id: UserId = id.parse()?;
        let module = ctx.data_unchecked::<AppModule>();
        let user_repository: Box<dyn UserRepositoryInterface + Send + Sync> =
            module.provide().unwrap();

        let result = user_repository.find(&user_id).await;

        if let Ok(user) = result {
            Ok(Some(GraphUser::build(user)))
        } else {
            Ok(None)
        }
    }

    #[tracing::instrument(skip(ctx))]
    async fn search_user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Search a user")] nickname: String,
    ) -> Option<GraphUser> {
        let module = ctx.data_unchecked::<AppModule>();
        let user_repository: Box<dyn UserRepositoryInterface + Send + Sync> =
            module.provide().unwrap();

        let result = user_repository.find_by_nickname(&nickname).await;

        if let Ok(user) = result {
            Some(GraphUser::build(user))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    #[tracing::instrument(skip(ctx))]
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        nickname: String,
    ) -> async_graphql::Result<GraphUser> {
        let module = ctx.data_unchecked::<AppModule>();
        let create_user: Box<dyn CreateUserUseCase> = module.provide().unwrap();
        Ok(create_user
            .call(&nickname)
            .await
            .map(|user| GraphUser::build(user))?)
    }

    #[tracing::instrument(skip(ctx))]
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        content: String,
    ) -> async_graphql::Result<GraphPost> {
        let module = ctx.data_unchecked::<AppModule>();
        let create_post: Box<dyn CreatePostUseCase> = module.provide().unwrap();
        Ok(create_post
            .call(&content)
            .await
            .map(|post| GraphPost::build(post))?)
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn interval(&self, #[graphql(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        tokio::time::interval(Duration::from_secs(1)).map(move |_| {
            value += n;
            value
        })
    }
}
