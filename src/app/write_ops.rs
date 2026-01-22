use crate::app::models::{CreateKVRequest, KeyValueResponse, Metadata, UpdateKVRequest};
use crate::types::Key;
use actix_web::{HttpResponse, Responder, delete, post, put, web};
use chrono::Utc;

#[post("/keys")]
pub async fn create_kv(body: web::Json<CreateKVRequest>) -> impl Responder {
    let now = Utc::now();
    let response = KeyValueResponse {
        key: body.key.clone(),
        value: body.value.clone(),
        metadata: Metadata {
            created_at: now,
            updated_at: now,
        },
    };

    HttpResponse::Ok().json(response)
}

#[put("/keys/{key}")]
pub async fn update_kv(key: web::Path<Key>, body: web::Json<UpdateKVRequest>) -> impl Responder {
    let now = Utc::now();
    let response = KeyValueResponse {
        key: key.clone(),
        value: body.value.clone(),
        metadata: Metadata {
            created_at: now,
            updated_at: now,
        },
    };

    HttpResponse::Ok().json(response)
}

#[delete("/keys/{key}")]
pub async fn delete_kv(key: web::Path<Key>) -> impl Responder {
    let now = Utc::now();
    let response = KeyValueResponse {
        key: key.clone(),
        value: "body.value.clone()".to_string(),
        metadata: Metadata {
            created_at: now,
            updated_at: now,
        },
    };

    HttpResponse::Ok().json(response)
}
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_kv).service(update_kv).service(delete_kv);
}
