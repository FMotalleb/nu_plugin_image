#[macro_export]
macro_rules! trace {
    ( #$tag:expr, $($args:tt)+) => {
        slog::log!(crate::logging::logger::INTERNAL_LOGGER, slog::Level::Trace, $tag, $($args)+)
    };
    ( $($args:tt)+) => {
        slog::log!(crate::logging::logger::INTERNAL_LOGGER, slog::Level::Trace, "", $($args)+)
    };
}

#[macro_export]
macro_rules! debug {
    ( #$tag:expr, $($args:tt)+) => {
        slog::log!(crate::logging::logger::INTERNAL_LOGGER, slog::Level::Debug, $tag, $($args)+)
    };
    ( $($args:tt)+) => {
        slog::log!(crate::logging::logger::INTERNAL_LOGGER, slog::Level::Debug, "", $($args)+)
    };
}
#[macro_export]
macro_rules! info {
    ( #$tag:expr, $($args:tt)+) => {
        slog::log!(crate::logging::logger::INTERNAL_LOGGER, slog::Level::Info, $tag, $($args)+)
    };
    ( $($args:tt)+) => {
        slog::log!(crate::logging::logger::INTERNAL_LOGGER, slog::Level::Info, "", $($args)+)
    };
}
#[macro_export]
macro_rules! warn(
    ( #$tag:expr, $($args:tt)+) => {
        slog::log!(crate::logging::logger::INTERNAL_LOGGER, slog::Level::Warning, $tag, $($args)+)
    };
    ( $($args:tt)+) => {
        slog::log!(crate::logging::logger::INTERNAL_LOGGER, slog::Level::Warning, "", $($args)+)
    };
);
#[macro_export]
macro_rules! error {
    ( #$tag:expr, $($args:tt)+) => {
        slog::log!(crate::logging::logger::INTERNAL_LOGGER, slog::Level::Error, $tag, $($args)+)
    };
    ( $($args:tt)+) => {
        slog::log!(crate::logging::logger::INTERNAL_LOGGER, slog::Level::Error, "", $($args)+)
    };
}
#[macro_export]
macro_rules! critical {
    ( #$tag:expr, $($args:tt)+) => {
        slog::log!(crate::logging::logger::INTERNAL_LOGGER, slog::Level::Critical, $tag, $($args)+)
    };
    ( $($args:tt)+) => {
        slog::log!(crate::logging::logger::INTERNAL_LOGGER, slog::Level::Critical, "", $($args)+)
    };
}
