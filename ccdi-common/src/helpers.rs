use log::error;
use std::fmt::Debug;

// ============================================ PUBLIC =============================================

pub fn to_string<T: Debug>(item: T) -> String {
    format!("{:?}", item)
}

pub fn log_err<E: Debug>(label: &str, result: Result<(), E>) {
    if let Err(error) = result {
        error!("Error in '{}': {}", label, to_string(error))
    }
}