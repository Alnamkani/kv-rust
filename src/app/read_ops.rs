use crate::app::models::{Metadata, ValueResponse};
use crate::types::Key;
use actix_web::{HttpResponse, Responder, get, web};
use chrono::Utc;

#[get("/keys/{key}")]
pub async fn get_value_by_key(key: web::Path<Key>) -> impl Responder {
    let now = Utc::now();
    let response = ValueResponse {
        value: key.into_inner().into_string(),
        metadata: Metadata {
            created_at: now,
            updated_at: now,
        },
    };

    HttpResponse::Ok().json(response)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_value_by_key);
}
