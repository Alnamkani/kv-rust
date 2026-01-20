use actix_web::{App, HttpServer};

mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server starting on http://localhost:8080");

    HttpServer::new(|| App::new().service(app::health::health))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
