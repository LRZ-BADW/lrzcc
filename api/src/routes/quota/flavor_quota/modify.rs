use actix_web::{
    web::{Data, Json, Path, ReqData},
    HttpResponse,
};
use anyhow::Context;
use lrzcc_wire::{
    quota::{FlavorQuota, FlavorQuotaModifyData},
    user::{Project, User},
};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::FlavorQuotaIdParam;
use crate::{
    authorization::require_admin_user,
    database::quota::flavor_quota::select_flavor_quota_from_db,
    error::{NotFoundOrUnexpectedApiError, OptionApiError},
};

#[tracing::instrument(name = "flavor_quota_modify")]
pub async fn flavor_quota_modify(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    data: Json<FlavorQuotaModifyData>,
    params: Path<FlavorQuotaIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    // TODO: do further validation
    if data.id != params.flavor_quota_id {
        return Err(OptionApiError::ValidationError(
            "ID in URL does not match ID in body".to_string(),
        ));
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavor_quota =
        update_flavor_quota_in_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_quota))
}

#[tracing::instrument(
    name = "update_flavor_quota_in_db",
    skip(data, transaction)
)]
pub async fn update_flavor_quota_in_db(
    transaction: &mut Transaction<'_, MySql>,
    data: &FlavorQuotaModifyData,
) -> Result<FlavorQuota, NotFoundOrUnexpectedApiError> {
    let row = select_flavor_quota_from_db(transaction, data.id as u64).await?;
    let user = data.user.unwrap_or(row.user);
    // TODO: what about the username
    let quota = data.quota.unwrap_or(row.quota);
    let flavor_group = data.flavor_group.unwrap_or(row.flavor_group);
    // TODO: what about the flavor group name
    let query1 = sqlx::query!(
        r#"
        UPDATE quota_quota
        SET
            user_id = ?,
            quota = ?
        WHERE id = ?
        "#,
        user,
        quota,
        data.id,
    );
    transaction
        .execute(query1)
        .await
        .context("Failed to execute first update query")?;
    let query2 = sqlx::query!(
        r#"
        UPDATE quota_flavorquota
        SET flavor_group_id = ?
        WHERE quota_ptr_id = ?
        "#,
        flavor_group,
        data.id,
    );
    transaction
        .execute(query2)
        .await
        .context("Failed to execute second update query")?;
    let flavor_quota = FlavorQuota {
        id: data.id,
        user,
        // TODO: we need to get the new username
        username: row.username,
        quota,
        flavor_group,
        // TODO: we need to get the new flavor group name
        flavor_group_name: row.flavor_group_name,
    };
    Ok(flavor_quota)
}
