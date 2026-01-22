use actix_web::{App, test, web};
use kv_rust::app::{health, read_ops, write_ops};
use kv_rust::service::{InMemoryStorage, Storage};
use std::sync::Arc;

fn create_test_app() -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let storage: Arc<dyn Storage + Send + Sync> = Arc::new(InMemoryStorage::new());
    let storage_data = web::Data::new(storage);

    App::new()
        .app_data(storage_data)
        .service(health::health)
        .configure(read_ops::configure)
        .configure(write_ops::configure)
}

#[actix_web::test]
async fn test_get_nonexistent_key_returns_404() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get()
        .uri("/keys/nonexistent")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 404);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["error"]["code"], "KEY_NOT_FOUND");
    assert!(
        body["error"]["message"]
            .as_str()
            .unwrap()
            .contains("nonexistent")
    );
}

#[actix_web::test]
async fn test_post_create_key_returns_201() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::post()
        .uri("/keys")
        .set_json(&serde_json::json!({
            "key": "test-key",
            "value": "test-value"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);
}

#[actix_web::test]
async fn test_post_duplicate_key_returns_409() {
    let app = test::init_service(create_test_app()).await;

    let create_req = test::TestRequest::post()
        .uri("/keys")
        .set_json(&serde_json::json!({
            "key": "duplicate-key",
            "value": "first-value"
        }))
        .to_request();

    test::call_service(&app, create_req).await;

    let duplicate_req = test::TestRequest::post()
        .uri("/keys")
        .set_json(&serde_json::json!({
            "key": "duplicate-key",
            "value": "second-value"
        }))
        .to_request();

    let resp = test::call_service(&app, duplicate_req).await;
    assert_eq!(resp.status().as_u16(), 409);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["error"]["code"], "KEY_ALREADY_EXISTS");
    assert!(
        body["error"]["message"]
            .as_str()
            .unwrap()
            .contains("duplicate-key")
    );
}

#[actix_web::test]
async fn test_full_crud_workflow() {
    let app = test::init_service(create_test_app()).await;

    let post_req = test::TestRequest::post()
        .uri("/keys")
        .set_json(&serde_json::json!({
            "key": "workflow-key",
            "value": "initial-value"
        }))
        .to_request();

    let resp = test::call_service(&app, post_req).await;
    assert_eq!(resp.status().as_u16(), 201);

    let get_req = test::TestRequest::get()
        .uri("/keys/workflow-key")
        .to_request();

    let resp = test::call_service(&app, get_req).await;
    assert_eq!(resp.status().as_u16(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["value"], "initial-value");

    let put_req = test::TestRequest::put()
        .uri("/keys/workflow-key")
        .set_json(&serde_json::json!({
            "value": "updated-value"
        }))
        .to_request();

    let resp = test::call_service(&app, put_req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let get_updated_req = test::TestRequest::get()
        .uri("/keys/workflow-key")
        .to_request();

    let resp = test::call_service(&app, get_updated_req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["value"], "updated-value");

    let delete_req = test::TestRequest::delete()
        .uri("/keys/workflow-key")
        .to_request();

    let resp = test::call_service(&app, delete_req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let get_after_delete_req = test::TestRequest::get()
        .uri("/keys/workflow-key")
        .to_request();

    let resp = test::call_service(&app, get_after_delete_req).await;
    assert_eq!(resp.status().as_u16(), 404);
}

#[actix_web::test]
async fn test_put_creates_new_key() {
    let app = test::init_service(create_test_app()).await;

    let put_req = test::TestRequest::put()
        .uri("/keys/new-key-via-put")
        .set_json(&serde_json::json!({
            "value": "created-via-put"
        }))
        .to_request();

    let resp = test::call_service(&app, put_req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let get_req = test::TestRequest::get()
        .uri("/keys/new-key-via-put")
        .to_request();

    let resp = test::call_service(&app, get_req).await;
    assert_eq!(resp.status().as_u16(), 200);
}

#[actix_web::test]
async fn test_delete_nonexistent_key_returns_404() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::delete()
        .uri("/keys/nonexistent")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 404);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["error"]["code"], "KEY_NOT_FOUND");
    assert!(
        body["error"]["message"]
            .as_str()
            .unwrap()
            .contains("nonexistent")
    );
}

#[actix_web::test]
async fn test_timestamp_preservation_on_update() {
    let app = test::init_service(create_test_app()).await;

    let post_req = test::TestRequest::post()
        .uri("/keys")
        .set_json(&serde_json::json!({
            "key": "timestamp-test",
            "value": "original"
        }))
        .to_request();

    let resp = test::call_service(&app, post_req).await;
    let create_body: serde_json::Value = test::read_body_json(resp).await;
    let original_created_at = create_body["metadata"]["created_at"].as_str().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(50));

    let put_req = test::TestRequest::put()
        .uri("/keys/timestamp-test")
        .set_json(&serde_json::json!({
            "value": "updated"
        }))
        .to_request();

    let resp = test::call_service(&app, put_req).await;
    let update_body: serde_json::Value = test::read_body_json(resp).await;
    let updated_created_at = update_body["metadata"]["created_at"].as_str().unwrap();
    let updated_at = update_body["metadata"]["updated_at"].as_str().unwrap();

    assert_eq!(
        original_created_at, updated_created_at,
        "created_at should be preserved on update"
    );
    assert!(
        updated_at > original_created_at,
        "updated_at should be newer than created_at"
    );
}
