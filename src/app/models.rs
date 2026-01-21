use crate::types::Key;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateKVRequest {
    pub key: Key,
    pub value: String,
}

// #[derive(Debug, Deserialize)] for now since the endpoint is a get endpoint forget about this
// pub struct GetKVRequest {
//     pub key: Key,
// }

#[derive(Debug, Deserialize)]
pub struct UpdateKVRequest {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValueResponse {
    pub key: Key,
    pub value: String,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueResponse {
    pub value: String,
    pub metadata: Metadata,
}

// #[derive(Debug, Serialize)]
// pub struct ErrorDetail {
//     pub code: String,
//     pub message: String,
// }

// #[derive(Debug, Serialize)]
// pub struct ErrorResponse {
//     pub error: ErrorDetail,
// }
