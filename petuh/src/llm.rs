pub mod petuh {
    #![allow(clippy::all)]
    tonic::include_proto!("petuh");
}

use anyhow::Result;
use tokio::sync::OnceCell;
use tonic::transport::Channel;

use crate::llm::petuh::{LlmRequest, Personality, petuh_llm_client::PetuhLlmClient};

static CLIENT: OnceCell<PetuhLlmClient<Channel>> = OnceCell::const_new();

pub struct LLMClient;

impl LLMClient {
    async fn get_client() -> PetuhLlmClient<Channel> {
        const ADDRESS: &str = "http://petuh-llm:50051";

        CLIENT
            .get_or_init(|| async {
                PetuhLlmClient::connect(ADDRESS)
                    .await
                    .unwrap_or_else(|err| panic!("Failed to connect to petuh-llm at {ADDRESS}. Error: {err}"))
            })
            .await
            .clone()
    }

    pub async fn request(personality: Personality, request: &str) -> Result<String> {
        let response = Self::get_client()
            .await
            .generate_response(LlmRequest {
                personality: personality as i32,
                query:       request.to_string(),
            })
            .await?;

        Ok(response.into_inner().response)
    }
}
