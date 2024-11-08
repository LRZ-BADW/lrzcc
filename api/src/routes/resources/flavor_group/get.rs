use super::FlavorGroupIdParam;
use crate::authorization::require_admin_user;
use crate::database::resources::flavor::select_minimal_flavors_by_group_from_db;
use crate::database::resources::flavor_group::select_flavor_group_from_db;
use crate::database::user::project::select_project_minimal_from_db;
use crate::error::OptionApiError;
use actix_web::web::{Data, Path, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::resources::FlavorGroupDetailed;
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

#[tracing::instrument(name = "flavor_group_get")]
pub async fn flavor_group_get(
    user: ReqData<User>,
    // TODO: not necessary?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<FlavorGroupIdParam>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    // TODO: this can all be condensed into one database function
    let flavor_group = select_flavor_group_from_db(
        &mut transaction,
        params.flavor_group_id as u64,
    )
    .await?;
    let flavors = select_minimal_flavors_by_group_from_db(
        &mut transaction,
        params.flavor_group_id as u64,
    )
    .await?;
    let project = select_project_minimal_from_db(
        &mut transaction,
        flavor_group.project as u64,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let flavor_group_detailed = FlavorGroupDetailed {
        id: flavor_group.id,
        name: flavor_group.name,
        flavors,
        project,
    };
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_group_detailed))
}
