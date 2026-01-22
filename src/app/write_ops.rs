use crate::app::models::{CreateKVRequest, UpdateKVRequest};
use crate::service::Storage;
use crate::types::Key;
use actix_web::{HttpResponse, Responder, delete, post, put, web, get};
use std::sync::Arc;

#[post("/keys")]
pub async fn create_kv(
    body: web::Json<CreateKVRequest>,
    storage: web::Data<Arc<dyn Storage + Send + Sync>>,
) -> impl Responder {
    let request = body.into_inner();

    if storage.contains_key(request.key.clone()) {
        return HttpResponse::Conflict().finish();
    }

    let response = storage.upsert(request);
    HttpResponse::Created().json(response)
}


#[get("/keys")]
pub async fn get_keys_list(
    storage: web::Data<Arc<dyn Storage + Send + Sync>>,
) -> impl Responder {
    HttpResponse::Ok().json(storage.list_keys())
}

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

#[delete("/keys/{key}")]
pub async fn delete_kv(
    key: web::Path<Key>,
    storage: web::Data<Arc<dyn Storage + Send + Sync>>,
) -> impl Responder {
    let key = key.into_inner();

    match storage.delete(key) {
        Some(value_response) => HttpResponse::Ok().json(value_response),
        None => HttpResponse::NotFound().finish(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_kv)
        .service(get_keys_list)
        .service(update_kv)
        .service(delete_kv);
}
