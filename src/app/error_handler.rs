use crate::app::models::{ErrorDetail, ErrorResponse};
use actix_web::{HttpResponse, error::JsonPayloadError};

pub fn json_error_handler(
    err: JsonPayloadError,
    _req: &actix_web::HttpRequest,
) -> actix_web::Error {
    let error_response = ErrorResponse {
        error: ErrorDetail {
            code: "VALIDATION_ERROR".to_string(),
            message: format!("{}", err),
        },
    };

    let response = HttpResponse::BadRequest().json(error_response);
    actix_web::error::InternalError::from_response(err, response).into()
}
