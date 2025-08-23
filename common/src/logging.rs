use tracing_appender::{non_blocking, non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub(crate) fn setup_logging(app: &'static str) -> anyhow::Result<(WorkerGuard, WorkerGuard)> {
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
