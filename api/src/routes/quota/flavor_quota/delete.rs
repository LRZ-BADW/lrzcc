use actix_web::{
    web::{Data, Path, ReqData},
    HttpResponse,
};
use anyhow::Context;
use avina_wire::user::{Project, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::FlavorQuotaIdParam;
use crate::{
    authorization::require_admin_user,
    error::{MinimalApiError, NormalApiError},
};

#[tracing::instrument(name = "flavor_quota_delete")]
pub async fn flavor_quota_delete(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<FlavorQuotaIdParam>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    delete_flavor_quota_from_db(
        &mut transaction,
        params.flavor_quota_id as u64,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::NoContent().finish())
}

#[tracing::instrument(name = "delete_flavor_quota_from_db", skip(transaction))]
async fn delete_flavor_quota_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_quota_id: u64,
) -> Result<(), MinimalApiError> {
    let query1 = sqlx::query!(
        r#"
        DELETE IGNORE FROM quota_flavorquota
        WHERE quota_ptr_id = ?
        "#,
        flavor_quota_id
    );
    let result1 = transaction
        .execute(query1)
        .await
        .context("Failed to execute delete query")?;
    if result1.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            // TODO: test that this message is really correct
            "Failed to delete flavor quota.".to_string(),
        ));
    }
    let query2 = sqlx::query!(
        r#"
        DELETE IGNORE FROM quota_quota
        WHERE id = ?
        "#,
        flavor_quota_id
    );
    let result2 = transaction
        .execute(query2)
        .await
        .context("Failed to execute delete query")?;
    if result2.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            // TODO: test that this message is really correct
            "Failed to delete quota.".to_string(),
        ));
    }
    Ok(())
}
