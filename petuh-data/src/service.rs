pub mod petuh {
    tonic::include_proto!("petuh");
}

use anyhow::Result;
use sercli::{Crud, FieldExtension, db::prepare_db};
use sqlx::PgPool;
use tonic::{Request, Response, Status};
use tracing::{info, instrument};

use crate::{
    entities::{Chat, ChatKind, SavedResponse, User},
    service::petuh::{
        AddUserResponse, Empty, GetResponsesResponse, GetUserRequest, petuh_data_server::PetuhData,
    },
};

type RpcSavedResponse = petuh::SavedResponse;
type RpcUser = petuh::User;
type RpcChat = petuh::Chat;

#[derive(Debug)]
pub struct PetuhDataService {
    pool: PgPool,
}

impl PetuhDataService {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            pool: prepare_db("../migrations").await?,
        })
    }

    async fn get_all_responses(&self) -> Result<Response<GetResponsesResponse>, Status> {
        let responses = SavedResponse::get_all(&self.pool)
            .await
            .or_else(|err| Err(Status::internal(err.to_string())))?;

        Ok(GetResponsesResponse {
            responses: responses.into_iter().map(Into::into).collect(),
        }
        .into())
    }
}

#[tonic::async_trait]
impl PetuhData for PetuhDataService {
    #[instrument]
    async fn get_responses(&self, _: Request<Empty>) -> Result<Response<GetResponsesResponse>, Status> {
        self.get_all_responses().await
    }

    #[instrument]
    async fn add_response(
        &self,
        request: Request<RpcSavedResponse>,
    ) -> Result<Response<GetResponsesResponse>, Status> {
        let response: SavedResponse = request.into_inner().into();

        info!(
            response = response.response,
            user_id = response.user_id,
            "adding response"
        );

        response
            .insert(&self.pool)
            .await
            .or_else(|err| Err(Status::internal(err.to_string())))?;

        self.get_all_responses().await
    }

    #[instrument]
    async fn remove_response(
        &self,
        request: Request<RpcSavedResponse>,
    ) -> Result<Response<GetResponsesResponse>, Status> {
        let response: SavedResponse = request.into_inner().into();

        SavedResponse::REQUEST
            .delete_where(response.request, &self.pool)
            .await
            .or_else(|err| Err(Status::internal(err.to_string())))?;

        info!(
            used_id = response.user_id,
            chat_id = response.chat_id,
            "deleted response"
        );

        self.get_all_responses().await
    }

    #[instrument]
    async fn add_user(&self, user: Request<RpcUser>) -> Result<Response<AddUserResponse>, Status> {
        let user: User = user.into_inner().into();

        if User::TELEGRAM_ID
            .one_where(user.telegram_id, &self.pool)
            .await
            .or_else(|err| Err(Status::internal(err.to_string())))?
            .is_some()
        {
            return Ok(Response::new(AddUserResponse { exists: true }));
        }

        info!(telegram_id = user.telegram_id, "adding new user");

        user.insert(&self.pool)
            .await
            .or_else(|err| Err(Status::internal(err.to_string())))?;

        Ok(Response::new(AddUserResponse { exists: false }))
    }

    #[instrument]
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> std::result::Result<Response<petuh::User>, Status> {
        let user_id: i64 = request.into_inner().user_id;

        let user = User::TELEGRAM_ID
            .one_where(user_id, &self.pool)
            .await
            .or_else(|err| Err(Status::internal(err.to_string())))?;

        let Some(user) = user else {
            return Err(Status::internal(
                format!("User with id: {user_id} doesn't exist",),
            ));
        };

        Ok(Response::new(user.into()))
    }

    #[instrument]
    async fn add_chat(&self, request: Request<RpcChat>) -> std::result::Result<Response<Empty>, Status> {
        let chat: Chat = request.into_inner().into();

        if Chat::TELEGRAM_ID
            .one_where(chat.telegram_id, &self.pool)
            .await
            .or_else(|err| Err(Status::internal(err.to_string())))?
            .is_some()
        {
            return Ok(Empty {}.into());
        };

        let chat = chat
            .insert(&self.pool)
            .await
            .or_else(|err| Err(Status::internal(err.to_string())))?;

        info!(chat_id = chat.telegram_id, "chat added");

        Ok(Empty {}.into())
    }
}

macro_rules! auto_from {
    ($a:ty, $b:ty, [ $($field:ident),* ]) => {
        impl From<$a> for $b {
            fn from(value: $a) -> Self {
                Self {
                    $( $field: value.$field.into() ),*,
                    ..Default::default()
                }
            }
        }

        impl From<$b> for $a {
            fn from(value: $b) -> Self {
                Self {
                    $( $field: value.$field.into() ),*,
                    ..Default::default()
                }
            }
        }
    };
}

auto_from!(
    SavedResponse,
    RpcSavedResponse,
    [user_id, chat_id, request, response]
);

auto_from!(
    User,
    RpcUser,
    [telegram_id, is_bot, first_name, username, nickname]
);

impl From<ChatKind> for i32 {
    fn from(value: ChatKind) -> Self {
        match value {
            ChatKind::Public => 0,
            ChatKind::Private => 1,
        }
    }
}

impl From<i32> for ChatKind {
    fn from(value: i32) -> Self {
        match value {
            0 => ChatKind::Public,
            1 => ChatKind::Private,
            _ => panic!("Invalid chat kind: {value}"),
        }
    }
}

auto_from!(Chat, RpcChat, [telegram_id, name, kind]);
