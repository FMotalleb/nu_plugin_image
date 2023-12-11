use std::sync::Mutex;

use include_flate::lazy_static;

use super::logger_state::*;

lazy_static! {
    pub static ref LOGGER: Mutex<LoggerState> = Mutex::new(LoggerState::new(false));
}
pub fn set_verbose(state: bool) {
    if let Ok(mut logger) = LOGGER.lock() {
        logger.set_verbose(state);
    }
}
pub fn vlog(message: String) {
    if let Ok(logger) = LOGGER.lock() {
        logger.log(&message);
    }
}
