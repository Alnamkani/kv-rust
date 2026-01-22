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
        description = r#"A lightweight, high-performance key-value store REST API built with Rust and Actix-web.

## Features
- Simple CRUD operations for string key-value pairs
- Automatic timestamp tracking (created_at, updated_at)
- Thread-safe in-memory storage with DashMap
- Comprehensive error handling with detailed error messages
- Request validation for keys and values

## Key Constraints
- Alphanumeric characters, hyphens (-), and underscores (_) only
- Length: 1-255 characters
- No whitespace or special characters

## Value Constraints
- Non-empty string
- No size limit (within reasonable memory constraints)

## Common Use Cases
- Session storage
- Configuration management
- Temporary data caching
- Feature flags
- User preferences"#
    )
)]
pub struct ApiDoc;
