use log::LevelFilter;
use simplelog::{CombinedLogger, Config, TerminalMode, ColorChoice, TermLogger};

pub fn init_logger() {
    let _result = CombinedLogger::init(
        vec![
            TermLogger::new(
                LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto
            ),
        ]
    );
}