use shaku::{module, Component, HasComponent, Interface};
use std::sync::Arc;

trait Logger: Interface {
    fn log(&self, content: &str);
}

trait DateLogger: Interface {
    fn log_date(&self);
}

#[derive(Component)]
#[shaku(interface = Logger)]
struct LoggerImpl;

impl Logger for LoggerImpl {
    fn log(&self, content: &str) {
        println!("{}", content);
    }
}

#[derive(Component)]
#[shaku(interface = DateLogger)]
struct DateLoggerImpl {
    #[shaku(inject)]
    logger: Arc<dyn Logger>,
    today: String,
    year: usize,
}

impl DateLogger for DateLoggerImpl {
    fn log_date(&self) {
        self.logger
            .log(&format!("Today is {}, {}", self.today, self.year));
    }
}

module! {
    MyModule {
        components = [LoggerImpl, DateLoggerImpl],
        providers = []
    }
}

pub fn main() {
    let module = MyModule::builder()
        .with_component_parameters::<DateLoggerImpl>(DateLoggerImplParameters {
            today: "Jan 26".to_string(),
            year: 2020,
        })
        .build();

    let date_logger: &dyn DateLogger = module.resolve_ref();
    date_logger.log_date();
}
