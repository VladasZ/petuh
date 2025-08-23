use sentry::{ClientInitGuard, ClientOptions, init};

use crate::Environment;

pub(crate) fn setup_sentry() -> anyhow::Result<ClientInitGuard> {
    dotenv::dotenv().expect("Failed to dotenv");

    let guard = init((
        std::env::var("SENTRY_LINK")?,
        ClientOptions {
            release: ::sentry::release_name!(),
            // Capture user IPs and potentially sensitive headers when using HTTP server integrations
            // see https://docs.sentry.io/platforms/rust/data-management/data-collected for more info
            send_default_pii: true,
            attach_stacktrace: true,
            environment: Some(Environment::string().into()),
            ..Default::default()
        },
    ));

    Ok(guard)
}
