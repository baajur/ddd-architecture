use super::repository::RepositoryInterface;
use shaku::Provider;

// Service
pub trait ServiceInterface {
    fn get_double(&self) -> usize;
}

#[derive(Provider)]
#[shaku(interface = ServiceInterface)]
pub struct Service {
    #[shaku(provide)]
    repo: Box<dyn RepositoryInterface>,
}

impl ServiceInterface for Service {
    fn get_double(&self) -> usize {
        2
    }
}
