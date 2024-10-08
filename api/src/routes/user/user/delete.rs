use crate::authorization::require_admin_user;
use crate::error::{MinimalApiError, NormalApiError};
use actix_web::web::{Data, Path, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{Project, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::UserIdParam;

#[tracing::instrument(name = "user_delete")]
pub async fn user_delete(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<UserIdParam>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    delete_user_from_db(&mut transaction, params.user_id as u64).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::NoContent().finish())
}

#[tracing::instrument(name = "delete_user_from_db", skip(transaction))]
pub async fn delete_user_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<(), MinimalApiError> {
    let query = sqlx::query!(
        r#"
        DELETE IGNORE FROM user_user
        WHERE id = ?
        "#,
        user_id
    );
    let result = transaction
        .execute(query)
        .await
        .context("Failed to execute delete query")?;
    if result.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            // TODO: test that this message is really correct
            "Failed to delete user, either it doesn't exist or other data still depends on it.".to_string(),
        ));
    }
    Ok(())
}
