pub mod petuh {
    tonic::include_proto!("petuh");
}

use tonic::{Request, Response, Status};

use crate::{
    chat_gpt::{query_denis, query_petuh, query_zul},
    service::petuh::{GenerateRequest, LlmResponse, Personality, petuh_llm_server::PetuhLlm},
};

#[derive(Default)]
pub struct PetuhLLMService;

#[tonic::async_trait]
impl PetuhLlm for PetuhLLMService {
    async fn generate_response(
        &self,
        request: Request<GenerateRequest>,
    ) -> Result<Response<LlmResponse>, Status> {
        let request = request.into_inner();

        let personality = Personality::try_from(request.personality)
            .or_else(|err| Err(Status::invalid_argument(err.to_string())))?;

        let response = query_response(personality, &request.query)
            .await
            .or_else(|err| Err(Status::internal(err.to_string())))?;

        Ok(LlmResponse { response }.into())
    }
}

async fn query_response(personality: Personality, query: &str) -> anyhow::Result<String> {
    Ok(match personality {
        Personality::Denis => query_denis(query).await?,
        Personality::Zyl => query_zul(query).await?,
        Personality::Petuh => query_petuh(query).await?,
    })
}
