use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

use crate::config::LogFormat;

pub fn init_tracing(log_level: String, log_format: LogFormat) {
    let reg = tracing_subscriber::registry();

    let reg = reg.with(EnvFilter::builder().parse_lossy(log_level));

    match log_format {
        super::config::LogFormat::JSON => reg.with(fmt::layer().json()).init(),
        super::config::LogFormat::Pretty => reg.with(fmt::layer().pretty()).init(),
        super::config::LogFormat::Compact => reg.with(fmt::layer().compact()).init(),
    };
}
