use crate::authorization::require_admin_user;
use crate::error::{MinimalApiError, NormalApiError};
use actix_web::web::{Data, Path, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{Project, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::ServerStateIdParam;

#[tracing::instrument(name = "server_state_delete")]
pub async fn server_state_delete(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<ServerStateIdParam>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    delete_server_state_from_db(
        &mut transaction,
        params.server_state_id as u64,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::NoContent().finish())
}

#[tracing::instrument(name = "delete_server_state_from_db", skip(transaction))]
async fn delete_server_state_from_db(
    transaction: &mut Transaction<'_, MySql>,
    server_state_id: u64,
) -> Result<(), MinimalApiError> {
    let query1 = sqlx::query!(
        r#"
        DELETE IGNORE FROM accounting_serverstate
        WHERE state_ptr_id = ?
        "#,
        server_state_id
    );
    let result1 = transaction
        .execute(query1)
        .await
        .context("Failed to execute delete query")?;
    if result1.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            // TODO: test that this message is really correct
            "Failed to delete server state.".to_string(),
        ));
    }
    let query2 = sqlx::query!(
        r#"
        DELETE IGNORE FROM accounting_state
        WHERE id = ?
        "#,
        server_state_id
    );
    let result2 = transaction
        .execute(query2)
        .await
        .context("Failed to execute delete query")?;
    if result2.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            // TODO: test that this message is really correct
            "Failed to delete state.".to_string(),
        ));
    }
    Ok(())
}
