use simplelog::{Config, SimpleLogger, CombinedLogger, LevelFilter, SharedLogger};

// ============================================ PUBLIC =============================================

pub fn init_logger(debug: bool) {
    let log_level = match debug {
        true => LevelFilter::Debug,
        false => LevelFilter::Info,
    };

    let loggers: Vec<Box<dyn SharedLogger>> = vec![
        SimpleLogger::new(log_level, Config::default())
    ];

    if let Err(error) = CombinedLogger::init(loggers) {
        eprintln!("Could not initialize logger: {:?}", error)
    }
}