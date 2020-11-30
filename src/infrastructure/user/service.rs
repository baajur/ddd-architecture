use super::super::super::domain::user::{RepositoryInterface, ServiceInterface};
use shaku::Provider;

#[derive(Provider)]
#[shaku(interface = ServiceInterface + Send + Sync)]
pub struct Service {
    #[shaku(provide)]
    repo: Box<dyn RepositoryInterface + Send + Sync>,
}

impl ServiceInterface for Service {
    fn get_double(&self) -> usize {
        2
    }
}
