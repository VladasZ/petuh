mod chat_gpt;
mod service;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello");

    pretty_env_logger::init();

    dotenv::dotenv().ok();

    let _guard = sentry::init((
        std::env::var("SENTRY_LINK")?,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            // Capture user IPs and potentially sensitive headers when using HTTP server integrations
            // see https://docs.sentry.io/platforms/rust/data-management/data-collected for more info
            send_default_pii: true,
            ..Default::default()
        },
    ));

    log::info!("Starting Telegram bot...");

    Ok(())
}
