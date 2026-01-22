use actix_web::{App, HttpServer, web};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

mod app;
mod service;
mod types;

use app::openapi::ApiDoc;
use service::{InMemoryStorage, Storage};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server starting on http://localhost:8080");
    println!("📚 API Documentation:");
    println!("   • Swagger UI: http://localhost:8080/swagger-ui");
    println!("   • Redoc:      http://localhost:8080/redoc");
    println!("   • RapiDoc:    http://localhost:8080/rapidoc");
    println!("   • Scalar:     http://localhost:8080/scalar");
    println!("📄 OpenAPI Spec: http://localhost:8080/api-docs/openapi.json");

    let storage: Arc<dyn Storage + Send + Sync> = Arc::new(InMemoryStorage::new());
    let storage_data = web::Data::new(storage);

    HttpServer::new(move || {
        App::new()
            .app_data(storage_data.clone())
            .app_data(
                web::JsonConfig::default().error_handler(app::error_handler::json_error_handler),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
            .service(Redoc::with_url("/redoc", ApiDoc::openapi()))
            .service(
                RapiDoc::new("/api-docs/openapi.json")
                    .path("/rapidoc")
            )
            .service(Scalar::with_url("/scalar", ApiDoc::openapi()))
            .service(app::health::health)
            .configure(app::read_ops::configure)
            .configure(app::write_ops::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
