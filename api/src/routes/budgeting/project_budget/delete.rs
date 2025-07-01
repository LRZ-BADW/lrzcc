use actix_web::{
    HttpResponse,
    web::{Data, Path, ReqData},
};
use anyhow::Context;
use avina_wire::user::User;
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::ProjectBudgetIdParam;
use crate::{
    authorization::require_admin_user,
    error::{MinimalApiError, NormalApiError},
};

#[tracing::instrument(name = "project_budget_delete")]
pub async fn project_budget_delete(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Path<ProjectBudgetIdParam>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    delete_project_budget_from_db(
        &mut transaction,
        params.project_budget_id as u64,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::NoContent().finish())
}

#[tracing::instrument(
    name = "delete_project_budget_from_db",
    skip(transaction)
)]
async fn delete_project_budget_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_budget_id: u64,
) -> Result<(), MinimalApiError> {
    let query = sqlx::query!(
        r#"
        DELETE IGNORE FROM budgeting_projectbudget
        WHERE id = ?
        "#,
        project_budget_id
    );
    let result = transaction
        .execute(query)
        .await
        .context("Failed to execute delete query")?;
    if result.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            // TODO: test that this message is really correct
            "Failed to delete project budget.".to_string(),
        ));
    }
    Ok(())
}
