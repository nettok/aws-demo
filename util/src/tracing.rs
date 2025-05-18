pub use tracing::*;

use std::{env, str::FromStr};
use tracing::metadata::LevelFilter;
use tracing_subscriber::EnvFilter;

pub fn init_tracing_default_subscriber() {
    let log_level_str = env::var("AWS_LAMBDA_LOG_LEVEL").or_else(|_| env::var("RUST_LOG"));
    let log_level =
        Level::from_str(log_level_str.as_deref().unwrap_or("INFO")).unwrap_or(Level::INFO);

    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::from_level(log_level).into())
                .from_env_lossy(),
        )
        .compact()
        .with_ansi(false)
        .init();
}
