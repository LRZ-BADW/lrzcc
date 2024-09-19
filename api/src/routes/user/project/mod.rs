use actix_web::web::{delete, get, patch, post, scope};
use actix_web::Scope;
use serde::Deserialize;
use sqlx::FromRow;

mod create;
use create::project_create;
mod list;
use list::project_list;
mod get;
use get::project_get;
mod modify;
use modify::project_modify;
mod delete;
use delete::project_delete;

// TODO use anyhow and thiserror

pub fn projects_scope() -> Scope {
    scope("/projects")
        .route("/", post().to(project_create))
        .route("", get().to(project_list))
        .route("/{project_id}", get().to(project_get))
        // TODO: what about PUT?
        .route("/{project_id}/", patch().to(project_modify))
        .route("/{project_id}/", delete().to(project_delete))
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
