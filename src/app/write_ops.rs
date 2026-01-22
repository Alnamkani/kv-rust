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
        (status = 409, description = "Key already exists", body = ErrorResponse)
    ),
    tag = "Keys - Write Operations"
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
        (status = 200, description = "List of all keys", body = Vec<String>)
    ),
    tag = "Keys - Read Operations"
)]
#[get("/keys")]
pub async fn get_keys_list(storage: web::Data<Arc<dyn Storage + Send + Sync>>) -> impl Responder {
    HttpResponse::Ok().json(storage.list_keys())
}

#[utoipa::path(
    put,
    path = "/keys/{key}",
    params(
        ("key" = String, Path, description = "Unique key identifier")
    ),
    request_body = UpdateKVRequest,
    responses(
        (status = 200, description = "Key-value pair updated or created", body = KeyValueResponse)
    ),
    tag = "Keys - Write Operations"
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
        ("key" = String, Path, description = "Unique key identifier")
    ),
    responses(
        (status = 200, description = "Key-value pair deleted successfully", body = ValueResponse),
        (status = 404, description = "Key not found", body = ErrorResponse)
    ),
    tag = "Keys - Write Operations"
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
