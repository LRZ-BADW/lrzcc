use actix_web::{
    web::{Data, Query, ReqData},
    HttpResponse,
};
use anyhow::Context;
use avina_wire::{
    resources::FlavorGroupListParams,
    user::{Project, User},
};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::resources::flavor_group::{
        select_all_flavor_groups_from_db, select_lrz_flavor_groups_from_db,
    },
    error::NormalApiError,
};

#[tracing::instrument(name = "flavor_group_list")]
pub async fn flavor_group_list(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<FlavorGroupListParams>,
) -> Result<HttpResponse, NormalApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavor_groups = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        select_all_flavor_groups_from_db(&mut transaction).await?
    } else {
        select_lrz_flavor_groups_from_db(&mut transaction).await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_groups))
}
