use include_flate::lazy_static;
use slog::{warn, Drain, Logger};
use slog_async::Async;
use slog_term::{CompactFormat, PlainSyncDecorator, TermDecorator};
use std::sync::{
    atomic::{self, AtomicU8},
    Arc, Mutex,
};

use crate::logging::runtime_filter::RuntimeLevelFilter;

lazy_static! {
    static ref INTERNAL_LOGGER: Logger = {
        let decorator = TermDecorator::new().build();
        let drain = CompactFormat::new(decorator).build().fuse();
        let drain = Async::new(drain).chan_size(10000).build().fuse();
        let drain = RuntimeLevelFilter {
            drain: drain,
            on: LOG_LEVEL.clone(),
        }
        .fuse();
        Logger::root(drain, slog::o!())
    };
    static ref LOG_LEVEL: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));
}
pub fn set_verbose(level: String) {
    // let plain = PlainSyncDecorator::new(std::io::stdout());
    // let logger = Logger::root(slog_term::FullFormat::new(plain).build().fuse(), o!());
    let level = match level.as_str() {
        "TRACE" | "trace" | "t" => 0,
        "DEBUG" | "debug" | "d" => 1,
        "INFO" | "info" | "i" => 2,
        "WARNING" | "warning" | "w" => 3,
        "ERROR" | "error" | "e" => 4,
        "CRITICAL" | "critical" | "c" => 5,
        _ => 2,
    };
    LOG_LEVEL.store(level, atomic::Ordering::Relaxed);

    // if let Ok(logger) = FILTERED_LOGGER.lock()
    //  {
    //     FILTERED_LOGGER = Some()
    //  }
}
pub fn vlog(message: String) {
    slog::warn!(INTERNAL_LOGGER, "{message}");
}

macro_rules! info {
    ($fmt:expr, $($arg:tt)*) => {
        slog::info!("{}", format!($fmt, $($arg)*));
    };
}
