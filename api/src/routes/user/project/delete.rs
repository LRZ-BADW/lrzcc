use actix_web::{
    web::{Data, Path, ReqData},
    HttpResponse,
};
use anyhow::Context;
use lrzcc_wire::user::{Project, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::ProjectIdParam;
use crate::{
    authorization::require_admin_user,
    error::{MinimalApiError, NormalApiError},
};

#[tracing::instrument(name = "project_delete")]
pub async fn project_delete(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<ProjectIdParam>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    delete_project_from_db(&mut transaction, params.project_id as u64).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::NoContent().finish())
}

#[tracing::instrument(name = "delete_project_from_db", skip(transaction))]
pub async fn delete_project_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<(), MinimalApiError> {
    let query = sqlx::query!(
        r#"
        DELETE IGNORE FROM user_project
        WHERE id = ?
        "#,
        project_id
    );
    let result = transaction
        .execute(query)
        .await
        .context("Failed to execute delete query")?;
    if result.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            // TODO: test that this message is really correct
            "Failed to delete project, either it doesn't exist or still has users or flavor groups.".to_string(),
        ));
    }
    Ok(())
}
