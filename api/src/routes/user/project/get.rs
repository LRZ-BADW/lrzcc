use super::{ProjectIdParam, ProjectRow};
use crate::error::not_found_error;
use actix_web::web::{Data, Path, ReqData};
use actix_web::HttpResponse;
use lrzcc_wire::user::{Project, ProjectDetailed, User};
use sqlx::MySqlPool;

// TODO proper query set and permissions
#[tracing::instrument(name = "project_get")]
pub async fn project_get(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<ProjectIdParam>,
) -> Result<HttpResponse, actix_web::Error> {
    let Ok(row) = sqlx::query_as!(
        ProjectRow,
        r#"
        SELECT
            id,
            name,
            openstack_id,
            user_class
        FROM user_project AS project
        WHERE
            project.id = ?
        "#,
        params.project_id,
    )
    .fetch_one(db_pool.get_ref())
    .await
    else {
        // TODO there might be other errors as well
        // TODO apply context and map_err
        return Err(not_found_error("Project with given ID not found"));
    };

    let project = ProjectDetailed {
        id: row.id as u32,
        name: row.name,
        openstack_id: row.openstack_id,
        user_class: row.user_class,
        // TODO retrieve actual values
        users: vec![],
        flavor_groups: vec![],
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(project))
}
