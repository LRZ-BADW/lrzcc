use actix_web::{
    web::{Data, Json, ReqData},
    HttpResponse,
};
use anyhow::Context;
use avina_wire::{
    quota::{FlavorQuota, FlavorQuotaCreateData},
    user::User,
};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::{
        quota::flavor_quota::insert_flavor_quota_into_db,
        resources::flavor_group::select_flavor_group_name_from_db,
        user::user::select_user_name_from_db,
    },
    error::OptionApiError,
};

#[tracing::instrument(name = "flavor_quota_create")]
pub async fn flavor_quota_create(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<FlavorQuotaCreateData>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let username =
        select_user_name_from_db(&mut transaction, data.user as u64).await?;
    let flavor_group_name = select_flavor_group_name_from_db(
        &mut transaction,
        data.flavor_group as u64,
    )
    .await?;
    let id = insert_flavor_quota_into_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let flavor_quota_created = FlavorQuota {
        id: id as u32,
        user: data.user,
        username,
        quota: data.quota,
        flavor_group: data.flavor_group,
        flavor_group_name,
    };
    Ok(HttpResponse::Created()
        .content_type("application/json")
        .json(flavor_quota_created))
}
