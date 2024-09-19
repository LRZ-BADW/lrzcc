use crate::authentication::require_admin_user;
use crate::error::not_found_error;
use actix_web::middleware::from_fn;
use actix_web::web::{delete, get, patch, post, scope, Data, Path, ReqData};
use actix_web::{HttpResponse, Scope};
use lrzcc_wire::user::{Project, User};
use serde::Deserialize;
use sqlx::{FromRow, MySqlPool};

mod create;
use create::project_create;
mod list;
use list::project_list;
mod get;
use get::project_get;
mod modify;
use modify::project_modify;

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
