use crate::types::Key;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Metadata {
    #[schema(example = "2026-01-22T10:30:00Z")]
    pub created_at: DateTime<Utc>,
    #[schema(example = "2026-01-22T15:45:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateKVRequest {
    #[schema(example = "user-123")]
    pub key: Key,
    #[schema(example = "John Doe")]
    pub value: String,
}

// #[derive(Debug, Deserialize)] for now since the endpoint is a get endpoint forget about this
// pub struct GetKVRequest {
//     pub key: Key,
// }

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateKVRequest {
    #[schema(example = "Jane Doe")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct KeyValueResponse {
    #[schema(example = "user-123")]
    pub key: Key,
    #[schema(example = "John Doe")]
    pub value: String,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ValueResponse {
    #[schema(example = "John Doe")]
    pub value: String,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorDetail {
    #[schema(example = "KEY_NOT_FOUND")]
    pub code: String,
    #[schema(example = "The requested key does not exist")]
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}
