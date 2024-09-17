use crate::authentication::require_admin_user;
use actix_web::middleware::from_fn;
use actix_web::web::ReqData;
use actix_web::web::{get, scope};
use actix_web::{HttpResponse, Scope};
use lrzcc_wire::hello::Hello;
use lrzcc_wire::user::{Project, User};

pub fn hello_scope() -> Scope {
    scope("/hello").route("", get().to(hello_user)).service(
        scope("")
            .wrap(from_fn(require_admin_user))
            .route("/admin", get().to(hello_admin)),
    )
}

#[tracing::instrument(name = "hello_user")]
pub async fn hello_user(
    user: ReqData<User>,
    project: ReqData<Project>,
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(Hello {
            message: format!(
                "Hello, {} from project {} with user class {}",
                user.name, project.name, project.user_class
            ),
        }))
}

#[tracing::instrument(name = "hello_admin")]
pub async fn hello_admin(
    user: ReqData<User>,
    project: ReqData<Project>,
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(Hello {
            message: format!(
                "Hello, admin {} from project {} with user class {}",
                user.name, project.name, project.user_class
            ),
        }))
}
