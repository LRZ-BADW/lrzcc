use super::ProjectIdParam;
use crate::authorization::require_admin_user;
use crate::database::resources::flavor_group::select_minimal_flavor_groups_by_project_id_from_db;
use crate::database::user::project::select_project_from_db;
use crate::database::user::user::select_minimal_users_by_project_id_from_db;
use crate::error::OptionApiError;
use actix_web::web::{Data, Path, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{Project, ProjectDetailed, ProjectRetrieved, User};
use sqlx::MySqlPool;

#[tracing::instrument(name = "project_get")]
pub async fn project_get(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<ProjectIdParam>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    if params.project_id != project.id {
        require_admin_user(&user)?;
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let row =
        select_project_from_db(&mut transaction, params.project_id as u64)
            .await?;

    let project = if user.is_staff {
        let users = select_minimal_users_by_project_id_from_db(
            &mut transaction,
            row.id as u64,
        )
        .await?;
        let flavor_groups = select_minimal_flavor_groups_by_project_id_from_db(
            &mut transaction,
            row.id as u64,
        )
        .await?;
        ProjectRetrieved::Detailed(ProjectDetailed {
            id: row.id as u32,
            name: row.name,
            openstack_id: row.openstack_id,
            user_class: row.user_class,
            users,
            flavor_groups,
        })
    } else {
        ProjectRetrieved::Normal(Project {
            id: row.id as u32,
            name: row.name,
            openstack_id: row.openstack_id,
            user_class: row.user_class,
        })
    };

    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(project))
}
