use crate::app::config::Config;

use anyhow::Result;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn init_tracing(config: Config) -> Result<()> {
    let env_filter = EnvFilter::new(config.log_level);

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(env_filter)
        .init();

    Ok(())
}
