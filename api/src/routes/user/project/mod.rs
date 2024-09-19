use crate::authentication::require_admin_user;
use crate::error::{bad_request_error, internal_server_error, not_found_error};
use actix_web::middleware::from_fn;
use actix_web::web::{
    delete, get, patch, post, scope, Data, Json, Path, ReqData,
};
use actix_web::{HttpResponse, Scope};
use lrzcc_wire::user::{Project, ProjectDetailed, ProjectModifyData, User};
use serde::Deserialize;
use sqlx::{Executor, FromRow, MySqlPool};

mod create;
use create::project_create;
mod list;
use list::project_list;

// TODO use anyhow and thiserror

pub fn projects_scope() -> Scope {
    scope("/projects").service(
        scope("")
            .wrap(from_fn(require_admin_user))
            .route("/", post().to(project_create))
            .route("", get().to(project_list))
            .route("/{project_id}", get().to(project_get))
            // TODO: what about PUT?
            .route("/{project_id}/", patch().to(project_modify))
            .route("/{project_id}/", delete().to(project_delete)),
    )
}

#[derive(Deserialize, Debug)]
struct ProjectIdParam {
    // TODO: why is this necessary, when this is clearly read in query_as
    #[allow(unused)]
    project_id: u32,
}

#[derive(Deserialize, FromRow, Debug)]
struct ProjectRow {
    id: i32,
    name: String,
    openstack_id: String,
    user_class: u32,
}

// TODO proper query set and permissions
#[tracing::instrument(name = "project_get")]
async fn project_get(
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

#[tracing::instrument(name = "project_modify")]
async fn project_modify(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    data: Json<ProjectModifyData>,
    params: Path<ProjectIdParam>,
) -> Result<HttpResponse, actix_web::Error> {
    if data.id != params.project_id {
        return Err(bad_request_error("ID in URL does not match ID in body"));
    }
    let Ok(mut transaction) = db_pool.begin().await else {
        // TODO there might be other errors as well
        // TODO apply context and map_err
        return Err(internal_server_error("Failed to start transaction"));
    };
    let query = sqlx::query_as::<_, ProjectRow>(
        r#"
        SELECT
            id,
            name,
            openstack_id,
            user_class
        FROM user_project
        WHERE id = ?
        "#,
    )
    .bind(data.id);
    let Ok(row) = transaction.fetch_one(query).await else {
        // TODO there might be other errors as well
        // TODO apply context and map_err
        return Err(not_found_error("Project with given ID not found"));
    };
    let Ok(row) = ProjectRow::from_row(&row) else {
        return Err(internal_server_error("Failed to parse project row"));
    };
    let name = data.name.clone().unwrap_or(row.name);
    let openstack_id = data.openstack_id.clone().unwrap_or(row.openstack_id);
    let user_class = data.user_class.unwrap_or(row.user_class);
    let query = sqlx::query!(
        r#"
        UPDATE user_project
        SET name = ?, openstack_id = ?, user_class = ?
        WHERE id = ?
        "#,
        name,
        openstack_id,
        user_class,
        data.id,
    );
    match transaction.execute(query).await {
        Ok(_) => (),
        Err(e) => {
            // TODO distinguish different database errors
            // TODO apply context and map_err
            tracing::error!("Failed to update project: {:?}", e);
            return Err(bad_request_error(
                "Failed to insert new project, maybe it already exists",
            ));
        }
    };
    if transaction.commit().await.is_err() {
        // TODO apply context and map_err
        return Err(internal_server_error("Failed to commit transaction"));
    }
    let project = Project {
        id: data.id,
        name,
        openstack_id,
        user_class,
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
