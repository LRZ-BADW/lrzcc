use actix_web::{
    web::{Data, Query, ReqData},
    HttpResponse,
};
use anyhow::Context;
use avina_wire::{
    quota::FlavorQuotaListParams,
    user::{Project, User},
};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::quota::flavor_quota::{
        select_all_flavor_quotas_from_db,
        select_flavor_quotas_by_flavor_group_from_db,
        select_flavor_quotas_by_user_from_db,
    },
    error::NormalApiError,
};

#[tracing::instrument(name = "flavor_quota_list")]
pub async fn flavor_quota_list(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<FlavorQuotaListParams>,
) -> Result<HttpResponse, NormalApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavor_quotas = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        select_all_flavor_quotas_from_db(&mut transaction).await?
    } else if let Some(user_id) = params.user {
        require_admin_user(&user)?;
        select_flavor_quotas_by_user_from_db(&mut transaction, user_id as u64)
            .await?
    } else if let Some(flavor_group_id) = params.group {
        require_admin_user(&user)?;
        select_flavor_quotas_by_flavor_group_from_db(
            &mut transaction,
            flavor_group_id as u64,
        )
        .await?
    } else {
        select_flavor_quotas_by_user_from_db(&mut transaction, user.id as u64)
            .await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_quotas))
}
