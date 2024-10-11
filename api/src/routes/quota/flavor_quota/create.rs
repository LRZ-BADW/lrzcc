use crate::authorization::require_admin_user;
use crate::database::{
    resources::flavor_group::select_flavor_group_name_from_db,
    user::user::select_user_name_from_db,
};
use crate::error::{MinimalApiError, OptionApiError};
use actix_web::web::{Data, Json, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::quota::{FlavorQuota, FlavorQuotaCreateData};
use lrzcc_wire::user::{Project, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

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

#[tracing::instrument(
    name = "insert_flavor_quota_into_db",
    skip(new_flavor_quota, transaction)
)]
pub async fn insert_flavor_quota_into_db(
    transaction: &mut Transaction<'_, MySql>,
    new_flavor_quota: &FlavorQuotaCreateData,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query1 = sqlx::query!(
        r#"
        INSERT IGNORE INTO quota_quota (quota, user_id)
        VALUES (?, ?)
        "#,
        new_flavor_quota.quota,
        new_flavor_quota.user
    );
    let result1 = transaction
        .execute(query1)
        .await
        .context("Failed to execute insert query")?;
    if result1.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new quota, a conflicting entry exists"
                .to_string(),
        ));
    }
    let id = result1.last_insert_id();
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query2 = sqlx::query!(
        r#"
        INSERT IGNORE INTO quota_flavorquota (quota_ptr_id, flavor_group_id)
        VALUES (?, ?)
        "#,
        id,
        new_flavor_quota.flavor_group
    );
    let result2 = transaction
        .execute(query2)
        .await
        .context("Failed to execute insert query")?;
    if result2.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new flavor quota, a conflicting entry exists"
                .to_string(),
        ));
    }
    Ok(id)
}
