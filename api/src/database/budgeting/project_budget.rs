use crate::error::{NotFoundOrUnexpectedApiError, UnexpectedOnlyError};
use anyhow::Context;
use lrzcc_wire::budgeting::ProjectBudget;
use sqlx::{Executor, FromRow, MySql, Transaction};

#[tracing::instrument(
    name = "select_maybe_project_budget_from_db",
    skip(transaction)
)]
pub async fn select_maybe_project_budget_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_budget_id: u64,
) -> Result<Option<ProjectBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, p.id as project, p.name as project_name, b.year, b.amount
        FROM budgeting_projectbudget as b, user_project as p
        WHERE
            b.project_id = p.id AND
            b.id = ?
        "#,
        project_budget_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    // TODO: isn't there a nicer way to write this?
    Ok(match row {
        Some(row) => Some(
            ProjectBudget::from_row(&row)
                .context("Failed to parse project_budget row")?,
        ),
        None => None,
    })
}

#[tracing::instrument(
    name = "select_project_budget_from_db",
    skip(transaction)
)]
pub async fn select_project_budget_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_budget_id: u64,
) -> Result<ProjectBudget, NotFoundOrUnexpectedApiError> {
    select_maybe_project_budget_from_db(transaction, project_budget_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError(
            "Project budget with given ID not found".to_string(),
        ))
}
