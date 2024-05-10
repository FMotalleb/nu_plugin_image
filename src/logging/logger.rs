use std::str::FromStr;

use nu_protocol::Value;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    filter::Targets, layer::SubscriberExt, util::SubscriberInitExt, FmtSubscriber,
};

pub fn init_logger(call: &nu_plugin::EvaluatedCall) {
    let builder = FmtSubscriber::builder();
    let builder = builder.with_max_level(LevelFilter::TRACE);

    let subscriber = builder.finish();
    let subscriber = {
        let targets = match call.get_flag_value("log-level") {
            Some(Value::String { val, .. }) => Targets::from_str(&val)
                .map_err(|e| {
                    eprintln!("Ignoring `log-level={:?}`: {}", val, e);
                })
                .unwrap_or_default(),

            _ => {
                eprintln!("using default value for `log-level` as INFO");
                Targets::from_str("INFO")
                    .map_err(|e| {
                        eprintln!("Ignoring `log-level=INFO`: {}", e);
                    })
                    .unwrap_or_default()
            }
        };
        subscriber.with(targets)
    };

    subscriber.init();
}
