use actix_web::body::BoxBody;
use actix_web::error::InternalError;
use actix_web::http::header::HeaderValue;
use actix_web::http::{header::CONTENT_TYPE, StatusCode};
use actix_web::HttpResponse;
use actix_web::ResponseError;
use lrzcc_wire::error::{error_chain_fmt, ErrorResponse};
use lrzcc_wire::user::User;

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

pub fn not_found_error(message: &str) -> actix_web::Error {
    InternalError::from_response(
        anyhow::anyhow!(message.to_string()),
        HttpResponse::BadRequest().json(ErrorResponse {
            detail: message.to_string(),
        }),
    )
    .into()
}

pub async fn not_found() -> Result<HttpResponse, actix_web::Error> {
    Err(not_found_error("This route does not exist."))
}

#[derive(thiserror::Error)]
pub enum NormalApiError {
    #[error("{0}")]
    ValidationError(String),
    #[error("{0}")]
    AuthorizationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for NormalApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for NormalApiError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let (status_code, message) = match self {
            NormalApiError::ValidationError(message) => {
                (StatusCode::BAD_REQUEST, message.clone())
            }
            NormalApiError::AuthorizationError(message) => {
                (StatusCode::FORBIDDEN, message.clone())
            }
            NormalApiError::UnexpectedError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error, contact admin or check logs"
                    .to_string(),
            ),
        };
        HttpResponse::build(status_code)
            .insert_header((
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            ))
            // TODO: handle unwrap
            .body(
                serde_json::to_string(&ErrorResponse { detail: message })
                    .unwrap(),
            )
    }
}

impl From<MinimalApiError> for NormalApiError {
    fn from(value: MinimalApiError) -> Self {
        match value {
            MinimalApiError::ValidationError(message) => {
                Self::ValidationError(message)
            }
            MinimalApiError::UnexpectedError(error) => {
                Self::UnexpectedError(error)
            }
        }
    }
}

#[derive(thiserror::Error)]
pub enum MinimalApiError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for MinimalApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(thiserror::Error)]
pub enum AuthOnlyError {
    #[error("{0}")]
    AuthorizationError(String),
}

impl std::fmt::Debug for AuthOnlyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for AuthOnlyError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let (status_code, message) = match self {
            AuthOnlyError::AuthorizationError(message) => {
                (StatusCode::FORBIDDEN, message.clone())
            }
        };
        HttpResponse::build(status_code)
            .insert_header((
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            ))
            // TODO: handle unwrap
            .body(
                serde_json::to_string(&ErrorResponse { detail: message })
                    .unwrap(),
            )
    }
}

impl From<AuthOnlyError> for NormalApiError {
    fn from(value: AuthOnlyError) -> Self {
        match value {
            AuthOnlyError::AuthorizationError(message) => {
                Self::AuthorizationError(message)
            }
        }
    }
}
