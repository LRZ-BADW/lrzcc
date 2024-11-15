use crate::error::{NotFoundOrUnexpectedApiError, UnexpectedOnlyError};
use anyhow::Context;
use lrzcc_wire::budgeting::UserBudget;
use sqlx::{Executor, FromRow, MySql, Transaction};

#[tracing::instrument(
    name = "select_maybe_user_budget_from_db",
    skip(transaction)
)]
pub async fn select_maybe_user_budget_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_budget_id: u64,
) -> Result<Option<UserBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, u.id as user, u.name as username, b.year, b.amount
        FROM budgeting_userbudget as b, user_user as u
        WHERE
            b.user_id = u.id AND
            b.id = ?
        "#,
        user_budget_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    // TODO: isn't there a nicer way to write this?
    Ok(match row {
        Some(row) => Some(
            UserBudget::from_row(&row)
                .context("Failed to parse user_budget row")?,
        ),
        None => None,
    })
}

#[tracing::instrument(name = "select_user_budget_from_db", skip(transaction))]
pub async fn select_user_budget_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_budget_id: u64,
) -> Result<UserBudget, NotFoundOrUnexpectedApiError> {
    select_maybe_user_budget_from_db(transaction, user_budget_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError(
            "User budget with given ID not found".to_string(),
        ))
}

#[tracing::instrument(
    name = "select_all_user_budgets_from_db",
    skip(transaction)
)]
pub async fn select_all_user_budgets_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<UserBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, u.id as user, u.name as username, b.year, b.amount
        FROM budgeting_userbudget as b, user_user as u
        WHERE b.user_id = u.id
        "#,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| UserBudget::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project budget")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_user_budgets_by_project_from_db",
    skip(transaction)
)]
pub async fn select_user_budgets_by_project_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Vec<UserBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, u.id as user, u.name as username, b.year, b.amount
        FROM budgeting_userbudget as b, user_user as u
        WHERE
            b.user_id = u.id AND
            u.project_id = ?
        "#,
        project_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| UserBudget::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project budget")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_user_budgets_by_user_from_db",
    skip(transaction)
)]
pub async fn select_user_budgets_by_user_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<Vec<UserBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, u.id as user, u.name as username, b.year, b.amount
        FROM budgeting_userbudget as b, user_user as u
        WHERE
            b.user_id = u.id AND
            u.id = ?
        "#,
        user_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| UserBudget::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project budget")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_user_budgets_by_year_from_db",
    skip(transaction)
)]
pub async fn select_user_budgets_by_year_from_db(
    transaction: &mut Transaction<'_, MySql>,
    year: u32,
) -> Result<Vec<UserBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, u.id as user, u.name as username, b.year, b.amount
        FROM budgeting_userbudget as b, user_user as u
        WHERE
            b.user_id = u.id AND
            b.year = ?
        "#,
        year
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| UserBudget::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project budget")?;
    Ok(rows)
}
