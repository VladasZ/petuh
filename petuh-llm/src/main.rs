mod chat_gpt;
mod service;

use anyhow::Result;
use common::initial_setup;
use tonic::transport::Server;
use tracing::info;

use crate::service::{PetuhLLMService, petuh::petuh_llm_server::PetuhLlmServer};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello");

    let _guard = initial_setup("petuh-llm")?;

    info!("Starting petuh-llm ...");

    let addr = "0.0.0.0:50051".parse()?;
    let service = PetuhLLMService;

    println!("petuh-llm server listening on {}", addr);

    Server::builder().add_service(PetuhLlmServer::new(service)).serve(addr).await?;

    Ok(())
}
