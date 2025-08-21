mod entities;
mod service;

use anyhow::Result;
use common::initial_setup;
use entities::*;
use tonic::transport::Server;
use tracing::info;

use crate::service::{PetuhDataService, petuh::petuh_data_server::PetuhDataServer};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello");

    let _guard = initial_setup("petuh-data")?;

    info!("Starting petuh-data");
    info!(connection_string = std::env::var("PG_CONNECTION_STRING")?);

    let addr = "0.0.0.0:50052".parse()?;
    let service = PetuhDataService::new().await?;

    info!("petuh-data server listening on {}", addr);

    Server::builder().add_service(PetuhDataServer::new(service)).serve(addr).await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use anyhow::Result;
    use sercli::db::{create_migration, prepare_db};

    use crate::entities::Model;

    #[ignore]
    #[tokio::test]
    async fn setup_db() -> Result<()> {
        use sercli::db::{generate_model, prepare_db};

        generate_model("../petuh-data/migrations")?;
        prepare_db("../petuh-data/migrations").await?;

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn wipe_db() -> Result<()> {
        let pool = prepare_db("../petuh-data/migrations").await?;
        Model::drop_all_tables(&pool).await?;
        sercli::db::stop_containers()
    }

    #[ignore]
    #[test]
    fn new_migration() -> Result<()> {
        create_migration("../petuh-data/migrations", "add_user_stats")
    }
}
