use domain::use_cases::create_user::{
    Interface as CreateUserUseCase, UseCase as CreateUserUseCaseImpl,
};
use infrastructure::{
    libs::db::service::DBService, user::db_repository::DbRepository as UserDbRepository,
};
use shaku::{module, HasProvider};

// Module
module! {
    AppModule {
        components = [],
        providers = [
            UserDbRepository,
            CreateUserUseCaseImpl,
            DBService,
        ]
    }
}

#[tokio::main]
pub async fn main() -> Result<(), ()> {
    let module = AppModule::builder().build();
    let create_user: Box<dyn CreateUserUseCase> = module.provide().unwrap();
    let user = create_user.call("Mathieu").await;
    println!("name: {:#?}", user);
    if let Err(err) = user {
        println!("err: {}", err);
    }

    Ok(())
}
