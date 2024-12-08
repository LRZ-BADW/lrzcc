use crate::authorization::require_admin_user;
use crate::database::quota::flavor_quota::insert_flavor_quota_into_db;
use crate::database::{
    resources::flavor_group::select_flavor_group_name_from_db,
    user::user::select_user_name_from_db,
};
use crate::error::OptionApiError;
use actix_web::web::{Data, Json, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::quota::{FlavorQuota, FlavorQuotaCreateData};
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

#[tracing::instrument(name = "flavor_quota_create")]
pub async fn flavor_quota_create(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
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
