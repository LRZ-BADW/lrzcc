use crate::openstack::{OpenStack, ProjectMinimal as OpenstackProjectMinimal};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::InternalError;
use actix_web::middleware::Next;
use actix_web::web::Data;
use actix_web::{HttpMessage, HttpResponse};
use sqlx::MySqlPool;

pub async fn require_valid_token(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let Some(token) = req.headers().get("X-Auth-Token") else {
        let response = HttpResponse::Unauthorized().finish();
        let e = anyhow::anyhow!("No token in request header");
        return Err(InternalError::from_response(e, response).into());
    };
    let Ok(token) = token.to_str() else {
        let response = HttpResponse::BadRequest().finish();
        let e = anyhow::anyhow!("Token is not a valid string");
        return Err(InternalError::from_response(e, response).into());
    };
    let Some(openstack) = req.app_data::<Data<OpenStack>>() else {
        let response = HttpResponse::InternalServerError().finish();
        let e = anyhow::anyhow!("No OpenStack client in application state");
        return Err(InternalError::from_response(e, response).into());
    };
    let Ok(os_project) = openstack.validate_user_token(token).await else {
        let response = HttpResponse::Unauthorized().finish();
        let e = anyhow::anyhow!("Failed to validate user token");
        return Err(InternalError::from_response(e, response).into());
    };
    req.extensions_mut().insert(os_project);
    next.call(req).await
}

pub async fn extract_user_and_project(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let Some(os_project) = req.extensions().get::<OpenstackProjectMinimal>()
    else {
        let response = HttpResponse::InternalServerError().finish();
        let e = anyhow::anyhow!("No OpenStack project in request extensions");
        return Err(InternalError::from_response(e, response).into());
    };
    let Some(db_pool) = req.app_data::<Data<MySqlPool>>() else {
        let response = HttpResponse::InternalServerError().finish();
        let e =
            anyhow::anyhow!("No database connection pool in application state");

        return Err(InternalError::from_response(e, response).into());
    };
    next.call(req).await
}
