use actix_web::{
    web::{Data, Query, ReqData},
    HttpResponse,
};
use anyhow::Context;
use lrzcc_wire::{
    resources::FlavorListParams,
    user::{Project, User},
};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::resources::flavor::{
        select_all_flavors_from_db, select_flavors_by_flavor_group_from_db,
        select_lrz_flavors_from_db,
    },
    error::NormalApiError,
};

#[tracing::instrument(name = "flavor_list")]
pub async fn flavor_list(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<FlavorListParams>,
) -> Result<HttpResponse, NormalApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavors = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        select_all_flavors_from_db(&mut transaction).await?
    } else if let Some(flavor_group_id) = params.group {
        require_admin_user(&user)?;
        select_flavors_by_flavor_group_from_db(
            &mut transaction,
            flavor_group_id as u64,
        )
        .await?
    } else {
        select_lrz_flavors_from_db(&mut transaction).await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavors))
}
