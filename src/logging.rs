use tracing_subscriber::{filter, prelude::*};

pub fn init() {
    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    let env_filter = filter::EnvFilter::builder()
        .with_default_directive(filter::LevelFilter::ERROR.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(stdout_log.with_filter(env_filter))
        .init();
}
