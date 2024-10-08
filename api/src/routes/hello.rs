use crate::authorization::require_admin_user;
use crate::error::AuthOnlyError;
use actix_web::web::ReqData;
use actix_web::web::{get, scope};
use actix_web::{HttpResponse, Scope};
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

#[tracing::instrument(name = "hello_admin")]
async fn hello_admin(
    user: ReqData<User>,
    project: ReqData<Project>,
) -> Result<HttpResponse, AuthOnlyError> {
    require_admin_user(&user)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(Hello {
            message: format!(
                "Hello, admin {} from project {} with user class {}",
                user.name, project.name, project.user_class
            ),
        }))
}
