use actix_web::error::InternalError;
use actix_web::HttpResponse;
use lrzcc_wire::error::ErrorResponse;

pub fn unauthorized_error(message: &str) -> actix_web::Error {
    InternalError::from_response(
        anyhow::anyhow!(message.to_string()),
        HttpResponse::Unauthorized().json(ErrorResponse {
            detail: message.to_string(),
        }),
    )
    .into()
}

pub fn internal_server_error(message: &str) -> actix_web::Error {
    InternalError::from_response(
        anyhow::anyhow!(message.to_string()),
        HttpResponse::InternalServerError().json(ErrorResponse {
            detail: message.to_string(),
        }),
    )
    .into()
}

pub fn bad_request_error(message: &str) -> actix_web::Error {
    InternalError::from_response(
        anyhow::anyhow!(message.to_string()),
        HttpResponse::BadRequest().json(ErrorResponse {
            detail: message.to_string(),
        }),
    )
    .into()
}
