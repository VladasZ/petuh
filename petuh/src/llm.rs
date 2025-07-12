pub mod petuh {
    tonic::include_proto!("petuh");
}

use anyhow::Result;
use tokio::sync::OnceCell;
use tonic::transport::Channel;

use crate::llm::petuh::{GenerateRequest, Personality, petuh_llm_client::PetuhLlmClient};

static CLIENT: OnceCell<PetuhLlmClient<Channel>> = OnceCell::const_new();

pub struct LLMClient;

impl LLMClient {
    async fn get_client() -> PetuhLlmClient<Channel> {
        CLIENT
            .get_or_init(|| async {
                PetuhLlmClient::connect("http://petuh-llm:50051")
                    .await
                    .expect("Failed to connect")
            })
            .await
            .clone()
    }

    pub async fn request(personality: Personality, request: &str) -> Result<String> {
        let response = Self::get_client()
            .await
            .generate_response(GenerateRequest {
                personality: personality as i32,
                query:       request.to_string(),
            })
            .await?;

        Ok(response.into_inner().response)
    }
}
