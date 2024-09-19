use actix_web::body::BoxBody;
use actix_web::http::header::HeaderValue;
use actix_web::http::{header::CONTENT_TYPE, StatusCode};
use actix_web::web::ReqData;
use actix_web::web::{get, scope};
use actix_web::ResponseError;
use actix_web::{HttpResponse, Scope};
use lrzcc_wire::error::{error_chain_fmt, ErrorResponse};
use lrzcc_wire::hello::Hello;
use lrzcc_wire::user::{Project, User};

pub fn hello_scope() -> Scope {
    scope("/hello")
        .route("", get().to(hello_user))
        .route("/admin", get().to(hello_admin))
}

#[tracing::instrument(name = "hello_user")]
async fn hello_user(
    user: ReqData<User>,
    project: ReqData<Project>,
) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(Hello {
            message: format!(
                "Hello, {} from project {} with user class {}",
                user.name, project.name, project.user_class
            ),
        })
}

#[derive(thiserror::Error)]
pub enum HelloAdminError {
    #[error("{0}")]
    AuthorizationError(String),
}

impl std::fmt::Debug for HelloAdminError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for HelloAdminError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let (status_code, message) = match self {
            HelloAdminError::AuthorizationError(message) => {
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

#[tracing::instrument(name = "hello_admin")]
async fn hello_admin(
    user: ReqData<User>,
    project: ReqData<Project>,
) -> Result<HttpResponse, HelloAdminError> {
    if !user.is_staff {
        return Err(HelloAdminError::AuthorizationError(
            "Admin privileges required".to_string(),
        ));
    }
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(Hello {
            message: format!(
                "Hello, admin {} from project {} with user class {}",
                user.name, project.name, project.user_class
            ),
        }))
}
