pub mod petuh {
    tonic::include_proto!("petuh");
}

use anyhow::Result;
use sercli::{Crud, FieldExtension, db::prepare_db};
use sqlx::PgPool;
use tonic::{Request, Response, Status};

use crate::{
    entities::SavedResponse,
    service::petuh::{Empty, GetResponsesResponse, petuh_responses_server::PetuhResponses},
};

type RpcSavedResponse = crate::service::petuh::SavedResponse;

pub struct PetuhResponsesService {
    pool: PgPool,
}

impl PetuhResponsesService {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            pool: prepare_db("petuh-responses/migrations").await?,
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
impl PetuhResponses for PetuhResponsesService {
    async fn get_responses(&self, _: Request<Empty>) -> Result<Response<GetResponsesResponse>, Status> {
        self.get_all_responses().await
    }

    async fn add_response(
        &self,
        request: Request<RpcSavedResponse>,
    ) -> Result<Response<GetResponsesResponse>, Status> {
        let response: SavedResponse = request.into_inner().into();

        response
            .insert(&self.pool)
            .await
            .or_else(|err| Err(Status::internal(err.to_string())))?;

        self.get_all_responses().await
    }

    async fn remove_response(
        &self,
        request: Request<RpcSavedResponse>,
    ) -> Result<Response<GetResponsesResponse>, Status> {
        let response: SavedResponse = request.into_inner().into();

        SavedResponse::REQUEST
            .delete_where(response.request, &self.pool)
            .await
            .or_else(|err| Err(Status::internal(err.to_string())))?;

        self.get_all_responses().await
    }
}

macro_rules! auto_from {
    ($a:ty, $b:ty, [ $($field:ident),* ]) => {
        impl From<$a> for $b {
            fn from(value: $a) -> Self {
                Self {
                    $( $field: value.$field ),*,
                    ..Default::default()
                }
            }
        }

        impl From<$b> for $a {
            fn from(value: $b) -> Self {
                Self {
                    $( $field: value.$field ),*,
                    ..Default::default()
                }
            }
        }
    };
}

auto_from!(SavedResponse, RpcSavedResponse, [request, response]);
