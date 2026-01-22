use crate::app::{health, models, read_ops, write_ops};
use crate::types::Key;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health,
        read_ops::get_value_by_key,
        write_ops::create_kv,
        write_ops::get_keys_list,
        write_ops::update_kv,
        write_ops::delete_kv,
    ),
    components(schemas(
        Key,
        models::CreateKVRequest,
        models::UpdateKVRequest,
        models::KeyValueResponse,
        models::ValueResponse,
        models::ErrorResponse,
        models::ErrorDetail,
        models::Metadata,
    )),
    tags(
        (name = "Health", description = "Service health check endpoints"),
        (name = "Keys - Read Operations", description = "Endpoints for reading key-value data"),
        (name = "Keys - Write Operations", description = "Endpoints for creating, updating, and deleting key-value data"),
    ),
    info(
        title = "KV-Rust API",
        version = "0.1.0",
        description = "A lightweight, high-performance key-value store REST API built with Rust and Actix-web. Provides simple CRUD operations for storing and retrieving string key-value pairs with metadata."
    )
)]
pub struct ApiDoc;
