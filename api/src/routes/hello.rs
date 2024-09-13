use crate::openstack::ProjectMinimal as OpenstackProjectMinimal;
use actix_web::web::ReqData;
use actix_web::HttpResponse;
use lrzcc_wire::hello::Hello;

#[tracing::instrument(name = "hello")]
pub async fn hello(
    os_project: ReqData<OpenstackProjectMinimal>,
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(Hello {
            message: format!("Hello, user {}!", os_project.name),
        }))
}
