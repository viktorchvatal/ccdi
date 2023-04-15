use log::error;
use std::fmt::Debug;

// ============================================ PUBLIC =============================================

pub fn to_string<T: Debug>(item: T) -> String {
    format!("{:?}", item)
}

pub fn log_err<T, E: Debug>(label: &str, result: Result<T, E>) -> Option<T> {
    match result {
        Err(error) => {
            error!("Error in '{}': {}", label, to_string(error));
            None
        },
        Ok(result) => Some(result)
    }
}