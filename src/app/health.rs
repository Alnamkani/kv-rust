use actix_web::{HttpResponse, Responder, get};

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = String)
    ),
    tag = "Health"
)]
#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};
    #[actix_web::test]
    async fn test_health_returns_ok() {
        // Create test app with health endpoint
        let app = test::init_service(App::new().service(health)).await;

        // Create GET request to /health
        let req = test::TestRequest::get().uri("/health").to_request();

        // Call the service
        let resp = test::call_service(&app, req).await;

        // Assert response is 200 OK
        assert!(resp.status().is_success());
        assert_eq!(resp.status().as_u16(), 200);

        // Assert body is "OK"
        let body = test::read_body(resp).await;
        assert_eq!(body, "OK");
    }
    #[actix_web::test]
    async fn test_health_wrong_method() {
        let app = test::init_service(App::new().service(health)).await;

        // Try POST instead of GET
        let req = test::TestRequest::post().uri("/health").to_request();

        let resp = test::call_service(&app, req).await;

        // Actix-web returns 404 when no route matches the method
        // (not 405 Method Not Allowed as you might expect)
        assert_eq!(resp.status().as_u16(), 404);
    }

    #[actix_web::test]
    async fn test_health_wrong_path() {
        let app = test::init_service(App::new().service(health)).await;

        // Try wrong path
        let req = test::TestRequest::get().uri("/wrong-path").to_request();

        let resp = test::call_service(&app, req).await;

        // Should be 404 Not Found
        assert_eq!(resp.status().as_u16(), 404);
    }
}
