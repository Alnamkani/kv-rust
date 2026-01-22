use crate::app::models::{CreateKVRequest, ErrorDetail, ErrorResponse, KeyValueResponse, UpdateKVRequest, ValueResponse};
use crate::service::Storage;
use crate::types::Key;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/keys",
    request_body = CreateKVRequest,
    responses(
        (status = 201, description = "Key-value pair created successfully", body = KeyValueResponse),
        (status = 409, description = "Key already exists - use PUT to update", body = ErrorResponse),
        (status = 400, description = "Invalid key format or empty value", body = ErrorResponse)
    ),
    tag = "Keys - Write Operations",
    summary = "Create new key-value pair",
    description = "Creates a new key-value pair in the store. The key must be unique and follow the naming constraints (alphanumeric, hyphens, underscores, 1-255 chars). Returns 409 if the key already exists. Use PUT to update existing keys."
)]
#[post("/keys")]
pub async fn create_kv(
    body: web::Json<CreateKVRequest>,
    storage: web::Data<Arc<dyn Storage + Send + Sync>>,
) -> impl Responder {
    let request = body.into_inner();

    match storage.insert(request) {
        Ok(response) => HttpResponse::Created().json(response),
        Err(storage_error) => {
            let error = ErrorResponse {
                error: ErrorDetail {
                    code: storage_error.error_code().to_string(),
                    message: storage_error.to_string(),
                },
            };
            HttpResponse::Conflict().json(error)
        }
    }
}

#[utoipa::path(
    get,
    path = "/keys",
    responses(
        (status = 200, description = "List of all keys in the store", body = Vec<String>, example = json!(["user-123", "config-prod", "session-abc"]))
    ),
    tag = "Keys - Read Operations",
    summary = "List all keys",
    description = "Returns an array of all keys currently stored in the key-value store. Useful for discovering what data is available or for administrative purposes."
)]
#[get("/keys")]
pub async fn get_keys_list(storage: web::Data<Arc<dyn Storage + Send + Sync>>) -> impl Responder {
    HttpResponse::Ok().json(storage.list_keys())
}

#[utoipa::path(
    put,
    path = "/keys/{key}",
    params(
        ("key" = String, Path, description = "Unique key identifier", example = "user-123")
    ),
    request_body = UpdateKVRequest,
    responses(
        (status = 200, description = "Key-value pair updated or created (idempotent upsert operation)", body = KeyValueResponse),
        (status = 400, description = "Invalid key format or empty value", body = ErrorResponse)
    ),
    tag = "Keys - Write Operations",
    summary = "Update or create key-value pair",
    description = "Updates an existing key-value pair or creates it if it doesn't exist (upsert operation). This is an idempotent operation. If updating, preserves the original created_at timestamp and updates the updated_at timestamp."
)]
#[put("/keys/{key}")]
pub async fn update_kv(
    path: web::Path<Key>,
    body: web::Json<UpdateKVRequest>,
    storage: web::Data<Arc<dyn Storage + Send + Sync>>,
) -> impl Responder {
    let key = path.into_inner();
    let update_request = body.into_inner();

    let request = CreateKVRequest {
        key,
        value: update_request.value,
    };

    let response = storage.upsert(request);
    HttpResponse::Ok().json(response)
}

#[utoipa::path(
    delete,
    path = "/keys/{key}",
    params(
        ("key" = String, Path, description = "Unique key identifier", example = "user-123")
    ),
    responses(
        (status = 200, description = "Key-value pair deleted successfully, returns the deleted value", body = ValueResponse),
        (status = 404, description = "Key not found - nothing to delete", body = ErrorResponse)
    ),
    tag = "Keys - Write Operations",
    summary = "Delete key-value pair",
    description = "Removes a key-value pair from the store and returns the deleted value with its metadata. Returns 404 if the key does not exist."
)]
#[delete("/keys/{key}")]
pub async fn delete_kv(
    key: web::Path<Key>,
    storage: web::Data<Arc<dyn Storage + Send + Sync>>,
) -> impl Responder {
    let key = key.into_inner();

    match storage.delete(key) {
        Ok(value_response) => HttpResponse::Ok().json(value_response),
        Err(storage_error) => {
            let error = ErrorResponse {
                error: ErrorDetail {
                    code: storage_error.error_code().to_string(),
                    message: storage_error.to_string(),
                },
            };
            HttpResponse::NotFound().json(error)
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_kv)
        .service(get_keys_list)
        .service(update_kv)
        .service(delete_kv);
}
