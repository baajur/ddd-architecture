use domain::{
    use_cases::create_user::{Interface as CreateUserUseCase, UseCase as CreateUserUseCaseImpl},
    user::{Repository, Service},
};
use shaku::{module, HasProvider};

mod domain;

// Module

module! {
    ExampleModule {
        components = [],
        providers = [
            Repository,
            Service,
            CreateUserUseCaseImpl
        ]
    }
}

pub fn main() {
    let module = ExampleModule::builder().build();
    let create_user: Box<dyn CreateUserUseCase> = module.provide().unwrap();
    let user = create_user.call("Mathieu");
    println!("name: {:#?}", user);
}
