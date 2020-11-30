use shaku::{module, Component, HasComponent, HasProvider, Interface, Module, Provider};
use std::cell::RefCell;
use std::error::Error;

// Traits

trait ConnectionPool: Interface {
    fn get(&self) -> DBConnection;
}

trait Repository {
    fn get(&self) -> usize;
}

trait Service {
    fn get_double(&self) -> usize;
}

// Structs

struct DBConnection(RefCell<usize>);

#[derive(Component)]
#[shaku(interface = ConnectionPool)]
struct DatabaseConnectionPool {
    #[shaku(default = 42)]
    value: usize,
}

#[derive(Provider)]
#[shaku(interface = Repository)]
struct RepositoryImpl {
    #[shaku(provide)]
    db: Box<DBConnection>,
}

#[derive(Provider)]
#[shaku(interface = Service)]
struct ServiceImpl {
    #[shaku(provide)]
    repo: Box<dyn Repository>,
}

// Trait implementations

impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
    type Interface = DBConnection;

    fn provide(module: &M) -> Result<Box<DBConnection>, Box<dyn Error + 'static>> {
        let pool: &dyn ConnectionPool = module.resolve_ref();
        Ok(Box::new(pool.get()))
    }
}

impl ConnectionPool for DatabaseConnectionPool {
    fn get(&self) -> DBConnection {
        DBConnection(RefCell::new(self.value))
    }
}

impl Repository for RepositoryImpl {
    fn get(&self) -> usize {
        *(*self.db).0.borrow()
    }
}

impl Service for ServiceImpl {
    fn get_double(&self) -> usize {
        self.repo.get() * 2
    }
}

// Module

module! {
    ExampleModule {
        components = [DatabaseConnectionPool],
        providers = [DBConnection, RepositoryImpl, ServiceImpl]
    }
}

pub fn main() {
    let module = ExampleModule::builder().build();
    let service: Box<dyn Service> = module.provide().unwrap();

    let value = service.get_double();
    println!("value: {}", value);
    assert_eq!(value, 84);
}
