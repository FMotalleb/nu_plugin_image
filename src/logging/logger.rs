use include_flate::lazy_static;
use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{CompactFormat, TermDecorator};
use std::sync::{
    atomic::{self, AtomicU8},
    Arc,
};

use crate::logging::runtime_filter::RuntimeLevelFilter;

lazy_static! {
    pub static ref INTERNAL_LOGGER: Logger = {
        let decorator = TermDecorator::new().build();
        let drain = CompactFormat::new(decorator).build().fuse();
        let drain = RuntimeLevelFilter {
            drain: drain,
            on: LOG_LEVEL.clone(),
        }
        .fuse();
        let drain = Async::new(drain).build().fuse();
        Logger::root(drain, slog::o!())
    };
    pub static ref LOG_LEVEL: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));
}

pub fn set_verbose(level: String) {
    let level_id = match level.as_str() {
        "TRACE" | "trace" | "t" => 0,
        "DEBUG" | "debug" | "d" => 1,
        "INFO" | "info" | "i" => 2,
        "WARNING" | "warning" | "w" => 3,
        "ERROR" | "error" | "e" => 4,
        "CRITICAL" | "critical" | "c" => 5,
        _ => 2,
    };

    LOG_LEVEL.store(level_id, atomic::Ordering::SeqCst);
}
