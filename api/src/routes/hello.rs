use actix_web::web::ReqData;
use actix_web::HttpResponse;
use lrzcc_wire::hello::Hello;
use lrzcc_wire::user::{Project, User};

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
