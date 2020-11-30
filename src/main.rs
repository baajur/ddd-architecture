use domain::use_cases::create_user::{
    Interface as CreateUserUseCase, UseCase as CreateUserUseCaseImpl,
};
use infrastructure::user::{
    db_repository::DbRepository as UserDbRepository, service::Service as UserService,
};
use shaku::{module, HasProvider};

mod domain;
mod infrastructure;

// Module
module! {
    AppModule {
        components = [],
        providers = [
            UserDbRepository,
            UserService,
            CreateUserUseCaseImpl
        ]
    }
}

#[tokio::main]
pub async fn main() {
    let module = AppModule::builder().build();
    let create_user: Box<dyn CreateUserUseCase> = module.provide().unwrap();
    let user = create_user.call("Mathieu").await;
    println!("name: {:#?}", user);
}
