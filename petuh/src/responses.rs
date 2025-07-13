use anyhow::Result;
use tokio::sync::OnceCell;
use tonic::transport::Channel;

use crate::llm::petuh::{Empty, SavedResponse, petuh_responses_client::PetuhResponsesClient};

static CLIENT: OnceCell<PetuhResponsesClient<Channel>> = OnceCell::const_new();

pub struct ResponsesClient;

impl ResponsesClient {
    async fn get_client() -> PetuhResponsesClient<Channel> {
        const ADDRESS: &str = "http://localhost:50051";

        CLIENT
            .get_or_init(|| async {
                PetuhResponsesClient::connect(ADDRESS).await.unwrap_or_else(|err| {
                    panic!("Failed to connect to petuh-responses at {ADDRESS}. Error: {err}")
                })
            })
            .await
            .clone()
    }

    pub async fn get_responses() -> Result<Vec<SavedResponse>> {
        let responses = Self::get_client().await.get_responses(Empty {}).await?;

        Ok(responses.into_inner().responses)
    }

    pub async fn add_response(response: SavedResponse) -> Result<Vec<SavedResponse>> {
        let responses = Self::get_client().await.add_response(response).await?;

        Ok(responses.into_inner().responses)
    }

    pub async fn remove_response(response: SavedResponse) -> Result<Vec<SavedResponse>> {
        let responses = Self::get_client().await.remove_response(response).await?;

        Ok(responses.into_inner().responses)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::{llm::petuh::SavedResponse, responses::ResponsesClient};

    #[ignore]
    #[tokio::test]
    async fn test_response_client() -> Result<()> {
        assert_eq!(ResponsesClient::get_responses().await?, vec![]);

        let response = SavedResponse {
            request:  "vlik".to_string(),
            response: "pth".to_string(),
        };

        assert_eq!(
            ResponsesClient::add_response(response.clone()).await?,
            vec![response.clone()]
        );

        assert_eq!(ResponsesClient::get_responses().await?, vec![response.clone()]);

        ResponsesClient::remove_response(response.clone()).await?;

        assert_eq!(ResponsesClient::get_responses().await?, vec![]);

        Ok(())
    }
}
