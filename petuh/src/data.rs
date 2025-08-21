use anyhow::Result;
use tokio::sync::OnceCell;
use tonic::transport::Channel;

use crate::llm::petuh::{
    AddUserResponse, Empty, GetUserRequest, SavedResponse, User, petuh_data_client::PetuhDataClient,
};

static CLIENT: OnceCell<PetuhDataClient<Channel>> = OnceCell::const_new();

pub struct DataClient;

impl DataClient {
    async fn get_client() -> PetuhDataClient<Channel> {
        const ADDRESS: &str = "http://petuh-data:50052";

        CLIENT
            .get_or_init(|| async {
                PetuhDataClient::connect(ADDRESS).await.unwrap_or_else(|err| {
                    panic!("Failed to connect to petuh-data at {ADDRESS}. Error: {err}")
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

    pub async fn _remove_response(response: SavedResponse) -> Result<Vec<SavedResponse>> {
        let responses = Self::get_client().await.remove_response(response).await?;

        Ok(responses.into_inner().responses)
    }

    pub async fn add_user(user: &teloxide::types::User) -> Result<AddUserResponse> {
        let response = Self::get_client()
            .await
            .add_user(User {
                telegram_id: i32::try_from(user.id.0).expect("Failed to convert user id to i32"),
                is_bot:      user.is_bot,
                first_name:  user.first_name.clone(),
                username:    user.username.clone(),
                nickname:    None,
            })
            .await?;

        Ok(response.into_inner())
    }

    pub async fn get_user(user_id: i32) -> Result<User> {
        let response = Self::get_client().await.get_user(GetUserRequest { user_id }).await?;

        Ok(response.into_inner())
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::{data::DataClient, llm::petuh::SavedResponse};

    #[ignore]
    #[tokio::test]
    async fn test_data_client() -> Result<()> {
        assert_eq!(DataClient::get_responses().await?, vec![]);

        let response = SavedResponse {
            id:       1,
            user_id:  20,
            request:  "vlik".to_string(),
            response: "pth".to_string(),
        };

        assert_eq!(
            DataClient::add_response(response.clone()).await?,
            vec![response.clone()]
        );

        assert_eq!(DataClient::get_responses().await?, vec![response.clone()]);

        DataClient::_remove_response(response.clone()).await?;

        assert_eq!(DataClient::get_responses().await?, vec![]);

        Ok(())
    }
}
