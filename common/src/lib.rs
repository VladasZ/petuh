mod environment;
mod redis;

use anyhow::Result;
pub use environment::Environment;
use sentry::ClientInitGuard;
use tracing_appender::{non_blocking, non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::redis::Redis;

pub fn initial_setup(app: &'static str) -> Result<(ClientInitGuard, (WorkerGuard, WorkerGuard))> {
    Redis::set_app_name(app);
    Ok((setup_sentry()?, setup_logging(app)?))
}

fn setup_sentry() -> Result<ClientInitGuard> {
    dotenv::dotenv().expect("Failed to dotenv");

    let guard = sentry::init((
        std::env::var("SENTRY_LINK")?,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            // Capture user IPs and potentially sensitive headers when using HTTP server integrations
            // see https://docs.sentry.io/platforms/rust/data-management/data-collected for more info
            send_default_pii: true,
            ..Default::default()
        },
    ));

    Ok(guard)
}

fn setup_logging(app: &'static str) -> Result<(WorkerGuard, WorkerGuard)> {
    std::fs::create_dir_all("logs")?;

    let file_appender = rolling::daily("logs", format!("{app}.log"));
    let (file_writer, guard) = non_blocking(file_appender);

    let (console_writer, guard2) = non_blocking(std::io::stdout());

    let filter = EnvFilter::new(format!("{}=trace", app.replace('-', "_")))
        .add_directive("common=trace".parse()?)
        .add_directive("warn".parse()?); // Show warnings from all crates

    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_writer(file_writer)
                .json()
                .with_target(true)
                .with_file(true)
                .with_line_number(true),
        )
        .with(fmt::layer().with_target(true).with_writer(console_writer).pretty())
        .init();

    Ok((guard, guard2))
}
