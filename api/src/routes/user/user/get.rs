use crate::error::{NotFoundOrUnexpectedApiError, UnexpectedOnlyError};
use anyhow::Context;
use lrzcc_wire::user::User;
use sqlx::{Executor, FromRow, MySql, Transaction};

#[tracing::instrument(name = "select_maybe_user_from_db", skip(transaction))]
pub async fn select_maybe_user_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<Option<User>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            id,
            name,
            openstack_id,
            project_id,
            role,
            is_staff,
            is_active
        FROM user_user AS user
        WHERE
            user.id = ?
        "#,
        user_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => {
            Some(User::from_row(&row).context("Failed to parse user row")?)
        }
        None => None,
    })
}

#[tracing::instrument(name = "select_user_from_db", skip(transaction))]
pub async fn select_user_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<User, NotFoundOrUnexpectedApiError> {
    select_maybe_user_from_db(transaction, user_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError(
            "User with given ID not found".to_string(),
        ))
}
