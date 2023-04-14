use simplelog::{Config, SimpleLogger, CombinedLogger, LevelFilter};

pub fn init_logger() {
    CombinedLogger::init(
        vec![SimpleLogger::new(LevelFilter::Debug, Config::default())]
    ).unwrap();
}