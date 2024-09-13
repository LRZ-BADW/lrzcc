use crate::openstack::{OpenStack, ProjectMinimal as OpenstackProjectMinimal};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::InternalError;
use actix_web::middleware::Next;
use actix_web::web::Data;
use actix_web::{HttpMessage, HttpResponse};
use lrzcc_wire::user::{Project, User};
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
    let os_project = match req.extensions().get::<OpenstackProjectMinimal>() {
        Some(os_project) => os_project.clone(),
        None => {
            let response = HttpResponse::InternalServerError().finish();
            let e =
                anyhow::anyhow!("No OpenStack project in request extensions");
            return Err(InternalError::from_response(e, response).into());
        }
    };
    let Some(db_pool) = req.app_data::<Data<MySqlPool>>() else {
        let response = HttpResponse::InternalServerError().finish();
        let e =
            anyhow::anyhow!("No database connection pool in application state");

        return Err(InternalError::from_response(e, response).into());
    };

    struct Row {
        user_id: i32,
        user_name: String,
        user_openstack_id: String,
        user_role: u32,
        user_is_staff: i8,
        user_is_active: i8,
        project_id: i32,
        project_name: String,
        project_openstack_id: String,
        project_user_class: u32,
    }

    let Ok(row) = sqlx::query_as!(
        Row,
        r#"
        SELECT
            user.id AS user_id,
            user.name AS user_name,
            user.openstack_id AS user_openstack_id,
            user.role AS user_role,
            user.is_staff AS user_is_staff,
            user.is_active AS user_is_active,
            project.id AS project_id,
            project.name AS project_name,
            project.openstack_id AS project_openstack_id,
            project.user_class AS project_user_class
        FROM user_user AS user, user_project AS project
        WHERE
            user.project_id = project.id AND
            user.id = ?
        "#,
        os_project.id
    )
    .fetch_one(db_pool.get_ref())
    .await
    else {
        let response = HttpResponse::Unauthorized().finish();
        let e = anyhow::anyhow!(
            "Failed to retrieve user and project from database"
        );
        return Err(InternalError::from_response(e, response).into());
    };

    let user = User {
        id: row.user_id as u32,
        name: row.user_name,
        openstack_id: row.user_openstack_id,
        project: row.project_id as u32,
        project_name: row.project_name.clone(),
        role: row.user_role,
        is_staff: row.user_is_staff != 0,
        is_active: row.user_is_active != 0,
    };
    let project = Project {
        id: row.project_id as u32,
        name: row.project_name,
        openstack_id: row.project_openstack_id,
        user_class: row.project_user_class,
    };

    req.extensions_mut().insert(Data::new(user));
    req.extensions_mut().insert(Data::new(project));

    next.call(req).await
}
