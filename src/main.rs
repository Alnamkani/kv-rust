use actix_web::{App, HttpServer};

mod app;
mod service;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server starting on http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(app::health::health)
            .configure(app::read_ops::configure)
            .configure(app::write_ops::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
