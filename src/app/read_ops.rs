use crate::app::models::{ErrorDetail, ErrorResponse, ValueResponse};
use crate::service::Storage;
use crate::types::Key;
use actix_web::{HttpResponse, Responder, get, web};
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/keys/{key}",
    params(
        ("key" = String, Path, description = "Unique key identifier (alphanumeric, hyphens, underscores, 1-255 chars)")
    ),
    responses(
        (status = 200, description = "Successfully retrieved value", body = ValueResponse),
        (status = 404, description = "Key not found", body = ErrorResponse)
    ),
    tag = "Keys - Read Operations"
)]
#[get("/keys/{key}")]
pub async fn get_value_by_key(
    key: web::Path<Key>,
    storage: web::Data<Arc<dyn Storage + Send + Sync>>,
) -> impl Responder {
    let key = key.into_inner();

    match storage.get(key) {
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
    cfg.service(get_value_by_key);
}
