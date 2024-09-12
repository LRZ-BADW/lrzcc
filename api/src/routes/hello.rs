use crate::openstack::ProjectMinimal;
use actix_web::web::ReqData;
use actix_web::HttpResponse;
use lrzcc_wire::hello::Hello;

#[tracing::instrument(name = "hello")]
pub async fn hello(
    project: ReqData<ProjectMinimal>,
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(Hello {
            message: format!("Hello, user {}!", project.name),
        }))
}
