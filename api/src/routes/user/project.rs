use crate::authentication::require_admin_user;
use crate::error::{not_found, not_found_error};
use actix_web::middleware::from_fn;
use actix_web::web::{delete, get, patch, post, scope, Data, Path, ReqData};
use actix_web::{HttpResponse, Scope};
use lrzcc_wire::user::{Project, ProjectDetailed, User};
use serde::Deserialize;
use sqlx::MySqlPool;

pub fn projects_scope() -> Scope {
    scope("/projects").service(
        scope("")
            .wrap(from_fn(require_admin_user))
            .route("", post().to(not_found))
            .route("", get().to(not_found))
            .route("/{project_id}", get().to(project_get))
            // TODO: what about PUT?
            .route("", patch().to(not_found))
            .route("/{project_id}/", delete().to(project_delete)),
    )
}

#[derive(Deserialize, Debug)]
struct ProjectIdParam {
    // TODO: why is this necessary, when this is clearly read in query_as
    #[allow(unused)]
    project_id: u32,
}

// TODO proper query set and permissions
#[tracing::instrument(name = "project_get")]
async fn project_get(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<ProjectIdParam>,
) -> Result<HttpResponse, actix_web::Error> {
    struct Row {
        id: i32,
        name: String,
        openstack_id: String,
        user_class: u32,
    }

    let Ok(row) = sqlx::query_as!(
        Row,
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

#[tracing::instrument(name = "project_delete")]
async fn project_delete(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<ProjectIdParam>,
) -> Result<HttpResponse, actix_web::Error> {
    if sqlx::query!(
        r#"DELETE FROM user_project WHERE id = ?"#,
        params.project_id,
    )
    .execute(db_pool.get_ref())
    .await
    .is_err()
    {
        // TODO there might be other errors as well
        // TODO apply context and map_err
        return Err(not_found_error("Project with given ID not found"));
    };

    Ok(HttpResponse::NoContent().finish())
}
