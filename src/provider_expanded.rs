#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;

#[macro_use]
extern crate std;
use shaku::{module, Component, HasComponent, HasProvider, Interface, Module, Provider};
use std::cell::RefCell;
use std::error::Error;

trait ConnectionPool: Interface {
    fn get(&self) -> DBConnection;
}

trait Repository {
    fn get(&self) -> usize;
}

trait Service {
    fn get_double(&self) -> usize;
}

struct DBConnection(RefCell<usize>);

# [ shaku ( interface = ConnectionPool ) ]
struct DatabaseConnectionPool {
    #[shaku(default = 42)]
    value: usize,
}

impl<M: ::shaku::Module> ::shaku::Component<M> for DatabaseConnectionPool {
    type Interface = dyn ConnectionPool;
    type Parameters = DatabaseConnectionPoolParameters;
    fn build(
        context: &mut ::shaku::ModuleBuildContext<M>,
        params: Self::Parameters,
    ) -> Box<Self::Interface> {
        Box::new(Self {
            value: params.value,
        })
    }
}

struct DatabaseConnectionPoolParameters {
    value: usize,
}

impl ::std::default::Default for DatabaseConnectionPoolParameters {
    #[allow(unreachable_code)]
    fn default() -> Self {
        Self { value: 42 }
    }
}

# [ shaku ( interface = Repository ) ]
struct RepositoryImpl {
    #[shaku(provide)]
    db: Box<DBConnection>,
}

impl<M: ::shaku::Module + ::shaku::HasProvider<DBConnection>> ::shaku::Provider<M>
    for RepositoryImpl
{
    type Interface = dyn Repository;
    fn provide(
        module: &M,
    ) -> ::std::result::Result<Box<Self::Interface>, Box<dyn::std::error::Error>> {
        Ok(Box::new(Self {
            db: module.provide()?,
        }))
    }
}

# [ shaku ( interface = Service ) ]
struct ServiceImpl {
    #[shaku(provide)]
    repo: Box<dyn Repository>,
}

impl<M: ::shaku::Module + ::shaku::HasProvider<dyn Repository>> ::shaku::Provider<M>
    for ServiceImpl
{
    type Interface = dyn Service;
    fn provide(
        module: &M,
    ) -> ::std::result::Result<Box<Self::Interface>, Box<dyn::std::error::Error>> {
        Ok(Box::new(Self {
            repo: module.provide()?,
        }))
    }
}

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

struct ExampleModule {
    __di_component_0:
        ::std::sync::Arc<<DatabaseConnectionPool as ::shaku::Component<Self>>::Interface>,
    __di_provider_0: ::std::sync::Arc<
        ::shaku::ProviderFn<Self, <DBConnection as ::shaku::Provider<Self>>::Interface>,
    >,
    __di_provider_1: ::std::sync::Arc<
        ::shaku::ProviderFn<Self, <RepositoryImpl as ::shaku::Provider<Self>>::Interface>,
    >,
    __di_provider_2: ::std::sync::Arc<
        ::shaku::ProviderFn<Self, <ServiceImpl as ::shaku::Provider<Self>>::Interface>,
    >,
}

impl ExampleModule {
    #[allow(bare_trait_objects)]
    fn builder() -> ::shaku::ModuleBuilder<Self> {
        ::shaku::ModuleBuilder::with_submodules(())
    }
}

impl ::shaku::Module for ExampleModule {
    #[allow(bare_trait_objects)]
    type Submodules = ();
    fn build(context: &mut ::shaku::ModuleBuildContext<Self>) -> Self {
        Self {
            __di_component_0: <Self as ::shaku::HasComponent<
                <DatabaseConnectionPool as ::shaku::Component<Self>>::Interface,
            >>::build_component(context),
            __di_provider_0: context.provider_fn::<DBConnection>(),
            __di_provider_1: context.provider_fn::<RepositoryImpl>(),
            __di_provider_2: context.provider_fn::<ServiceImpl>(),
        }
    }
}

impl ::shaku::HasComponent<<DatabaseConnectionPool as ::shaku::Component<Self>>::Interface>
    for ExampleModule
{
    fn build_component(
        context: &mut ::shaku::ModuleBuildContext<Self>,
    ) -> ::std::sync::Arc<<DatabaseConnectionPool as ::shaku::Component<Self>>::Interface> {
        context.build_component::<DatabaseConnectionPool>()
    }
    fn resolve(
        &self,
    ) -> ::std::sync::Arc<<DatabaseConnectionPool as ::shaku::Component<Self>>::Interface> {
        ::std::sync::Arc::clone(&self.__di_component_0)
    }
    fn resolve_ref(&self) -> &<DatabaseConnectionPool as ::shaku::Component<Self>>::Interface {
        ::std::sync::Arc::as_ref(&self.__di_component_0)
    }
    fn resolve_mut(
        &mut self,
    ) -> ::std::option::Option<&mut <DatabaseConnectionPool as ::shaku::Component<Self>>::Interface>
    {
        ::std::sync::Arc::get_mut(&mut self.__di_component_0)
    }
}

impl ::shaku::HasProvider<<DBConnection as ::shaku::Provider<Self>>::Interface> for ExampleModule {
    fn provide(
        &self,
    ) -> ::std::result::Result<
        ::std::boxed::Box<<DBConnection as ::shaku::Provider<Self>>::Interface>,
        ::std::boxed::Box<dyn::std::error::Error>,
    > {
        (self.__di_provider_0)(self)
    }
}

impl ::shaku::HasProvider<<RepositoryImpl as ::shaku::Provider<Self>>::Interface>
    for ExampleModule
{
    fn provide(
        &self,
    ) -> ::std::result::Result<
        ::std::boxed::Box<<RepositoryImpl as ::shaku::Provider<Self>>::Interface>,
        ::std::boxed::Box<dyn::std::error::Error>,
    > {
        (self.__di_provider_1)(self)
    }
}

impl ::shaku::HasProvider<<ServiceImpl as ::shaku::Provider<Self>>::Interface> for ExampleModule {
    fn provide(
        &self,
    ) -> ::std::result::Result<
        ::std::boxed::Box<<ServiceImpl as ::shaku::Provider<Self>>::Interface>,
        ::std::boxed::Box<dyn::std::error::Error>,
    > {
        (self.__di_provider_2)(self)
    }
}

pub fn main() {
    let module = ExampleModule::builder().build();
    let service: Box<dyn Service> = module.provide().unwrap();
    let value = service.get_double();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["value: ", "\n"],
            &match (&value,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(
                    arg0,
                    ::core::fmt::Display::fmt,
                )],
            },
        ));
    };
    {
        match (&value, &84) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                            &[
                                "assertion failed: `(left == right)`\n  left: `",
                                "`,\n right: `",
                                "`",
                            ],
                            &match (&&*left_val, &&*right_val) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                ],
                            },
                        ))
                    }
                }
            }
        }
    };
}
