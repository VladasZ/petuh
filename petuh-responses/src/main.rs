mod entities;
mod service;

use anyhow::Result;
use tonic::transport::Server;

use crate::service::{PetuhResponsesService, petuh::petuh_responses_server::PetuhResponsesServer};

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

    log::info!("Starting petuh-llm ...");

    let addr = "0.0.0.0:50051".parse()?;
    let service = PetuhResponsesService::new().await?;

    println!("petuh-responses server listening on {}", addr);

    Server::builder()
        .add_service(PetuhResponsesServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use anyhow::Result;
    use sercli::db::prepare_db;

    use crate::entities::Model;

    #[ignore]
    #[tokio::test]
    async fn setup_db() -> Result<()> {
        use sercli::db::{generate_model, prepare_db};

        generate_model("../petuh-responses/migrations")?;
        prepare_db("../petuh-responses/migrations").await?;

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn wipe_db() -> Result<()> {
        let pool = prepare_db("../petuh-responses/migrations").await?;
        Model::drop_all_tables(&pool).await?;
        sercli::db::stop_containers()
    }
}
