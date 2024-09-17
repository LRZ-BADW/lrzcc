use crate::authentication::require_admin_user;
use crate::error::{
    bad_request_error, internal_server_error, not_found, not_found_error,
};
use actix_web::middleware::from_fn;
use actix_web::web::{
    delete, get, patch, post, scope, Data, Json, Path, ReqData,
};
use actix_web::{HttpResponse, Scope};
use lrzcc_wire::user::{
    Project, ProjectCreateData, ProjectCreated, ProjectDetailed, User,
};
use serde::Deserialize;
use sqlx::{Executor, MySqlPool};

// TODO use anyhow and thiserror

pub fn projects_scope() -> Scope {
    scope("/projects").service(
        scope("")
            .wrap(from_fn(require_admin_user))
            .route("", post().to(project_create))
            .route("", get().to(project_list))
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

struct ProjectRow {
    id: i32,
    name: String,
    openstack_id: String,
    user_class: u32,
}

#[tracing::instrument(name = "project_create")]
async fn project_create(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    data: Json<ProjectCreateData>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_class = data.user_class.unwrap_or(1);
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query = sqlx::query!(
        r#"
        INSERT INTO user_project (name, openstack_id, user_class)
        VALUES (?, ?, ?)
        "#,
        data.name,
        data.openstack_id,
        data.user_class
    );
    let Ok(result) = db_pool.execute(query).await else {
        // TODO distinguish different database errors
        // TODO apply context and map_err
        return Err(bad_request_error(
            "Failed to insert new project, maybe it already exists.",
        ));
    };
    let id = result.last_insert_id();
    let project = ProjectCreated {
        id: id as u32,
        name: data.name.clone(),
        openstack_id: data.openstack_id.clone(),
        user_class,
        // TODO retrieve actual values
        users: vec![],
        flavor_groups: vec![],
    };
    Ok(HttpResponse::Created()
        .content_type("application/json")
        .json(project))
}

// TODO proper query set and permissions
#[tracing::instrument(name = "project_list")]
async fn project_list(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let Ok(rows) = sqlx::query_as!(
        ProjectRow,
        r#"
        SELECT
            id,
            name,
            openstack_id,
            user_class
        FROM user_project
        "#,
    )
    .fetch_all(db_pool.get_ref())
    .await
    else {
        // TODO there might be other errors as well
        // TODO apply context and map_err
        return Err(internal_server_error("Failed to retrieve projects"));
    };

    let projects = rows
        .into_iter()
        .map(|r| Project {
            id: r.id as u32,
            name: r.name,
            openstack_id: r.openstack_id,
            user_class: r.user_class,
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(projects))
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
