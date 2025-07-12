pub mod petuh {
    tonic::include_proto!("petuh");
}

use tonic::{Request, Response, Status};

use crate::{
    chat_gpt::query_denis,
    service::petuh::{GenerateRequest, LlmResponse, Personality, petuh_llm_server::PetuhLlm},
};

#[derive(Default)]
pub struct PetuhLLMService {}

#[tonic::async_trait]
impl PetuhLlm for PetuhLLMService {
    async fn generate_response(
        &self,
        request: Request<GenerateRequest>,
    ) -> Result<Response<LlmResponse>, Status> {
        let request = request.into_inner();

        match Personality::try_from(request.personality) {
            Ok(Personality::Denis) => query_denis(&request.query).await?,
            Ok(Personality::Zyl) => { /* handle Zyl */ }
            Ok(Personality::Petuh) => { /* handle Petuh */ }
            Err(err) => return Err(Status::invalid_argument(err.to_string())),
        }

        todo!()
    }
}
