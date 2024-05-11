use std::{
    result,
    sync::{
        atomic::{self, Ordering},
        Arc,
    },
};

use slog::Drain;

/// Custom Drain logic
pub struct RuntimeLevelFilter<D> {
    pub drain: D,
    pub on: Arc<atomic::AtomicU8>,
}

unsafe impl<D> Sync for RuntimeLevelFilter<D> {}
impl<D> Drain for RuntimeLevelFilter<D>
where
    D: Drain,
{
    type Ok = Option<D::Ok>;
    type Err = Option<D::Err>;

    fn log(
        &self,
        record: &slog::Record,
        values: &slog::OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err> {
        let level_id = self.on.load(Ordering::SeqCst);
        let current_level = match level_id {
            0 => slog::Level::Trace,
            1 => slog::Level::Debug,
            2 => slog::Level::Info,
            3 => slog::Level::Warning,
            4 => slog::Level::Error,
            5 => slog::Level::Critical,
            _ => slog::Level::Info,
        };
        if record.level().is_at_least(current_level) {
            self.drain.log(record, values).map(Some).map_err(Some)
        } else {
            Ok(None)
        }
    }
}
