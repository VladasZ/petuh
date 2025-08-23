use std::any::Any;

use tracing::info;

use crate::{Environment, logging::setup_logging, redis::Redis, sentry::setup_sentry};

pub fn initial_setup(app: &'static str) -> anyhow::Result<(impl Any, (impl Any, impl Any))> {
    let log_guard = setup_logging(app)?;
    Redis::set_app_name(app)?;
    let result = Ok((setup_sentry()?, log_guard));

    info!(
        app_name = Redis::app_name(),
        environment = Environment::string(),
        version = env!("CARGO_PKG_VERSION"),
        "started"
    );

    result
}
