use actix_web::{App, HttpServer, web};
use std::sync::Arc;

mod app;
mod service;
mod types;

use service::{InMemoryStorage, Storage};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server starting on http://localhost:8080");

    let storage: Arc<dyn Storage + Send + Sync> = Arc::new(InMemoryStorage::new());
    let storage_data = web::Data::new(storage);

    HttpServer::new(move || {
        App::new()
            .app_data(storage_data.clone())
            .service(app::health::health)
            .configure(app::read_ops::configure)
            .configure(app::write_ops::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
