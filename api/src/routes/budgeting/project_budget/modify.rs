use actix_web::{
    HttpResponse,
    web::{Data, Json, Path, ReqData},
};
use anyhow::Context;
use avina_wire::{
    budgeting::{ProjectBudget, ProjectBudgetModifyData},
    user::User,
};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::ProjectBudgetIdParam;
use crate::{
    authorization::require_admin_user,
    database::budgeting::project_budget::select_project_budget_from_db,
    error::{NotFoundOrUnexpectedApiError, OptionApiError},
};

#[tracing::instrument(name = "project_budget_modify")]
pub async fn project_budget_modify(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<ProjectBudgetModifyData>,
    params: Path<ProjectBudgetIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    // TODO: allow master user access
    // TODO: check that cost is below
    // TODO: handle force option
    require_admin_user(&user)?;
    // TODO: do further validation
    if data.id != params.project_budget_id {
        return Err(OptionApiError::ValidationError(
            "ID in URL does not match ID in body".to_string(),
        ));
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let project_budget =
        update_project_budget_in_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(project_budget))
}

#[tracing::instrument(
    name = "update_project_budget_in_db",
    skip(data, transaction)
)]
pub async fn update_project_budget_in_db(
    transaction: &mut Transaction<'_, MySql>,
    data: &ProjectBudgetModifyData,
) -> Result<ProjectBudget, NotFoundOrUnexpectedApiError> {
    let row =
        select_project_budget_from_db(transaction, data.id as u64).await?;
    let amount = data.amount.unwrap_or(row.amount);
    let query = sqlx::query!(
        r#"
        UPDATE budgeting_projectbudget
        SET amount = ?
        WHERE id = ?
        "#,
        amount,
        data.id,
    );
    transaction
        .execute(query)
        .await
        .context("Failed to execute update query")?;
    let project = ProjectBudget {
        id: data.id,
        amount,
        project: row.project,
        project_name: row.project_name,
        year: row.year,
    };
    Ok(project)
}
