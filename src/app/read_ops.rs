use crate::types::Key;
use actix_web::{HttpResponse, Responder, get, web};

#[get("/read/{key}")]
pub async fn get_value_by_key(key: web::Path<Key>) -> impl Responder {
    HttpResponse::Ok().body(key.into_inner().into_string())
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_value_by_key);
}
