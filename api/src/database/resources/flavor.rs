use crate::error::{NotFoundOrUnexpectedApiError, UnexpectedOnlyError};
use anyhow::Context;
use sqlx::{Executor, FromRow, MySql, Transaction};

#[tracing::instrument(
    name = "select_maybe_flavor_name_from_db",
    skip(transaction)
)]
pub async fn select_maybe_flavor_name_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_id: u64,
) -> Result<Option<String>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    #[allow(dead_code)]
    struct Row {
        name: String,
    }
    let query = sqlx::query!(
        r#"
        SELECT name
        FROM resources_flavor AS flavor
        WHERE flavor.id = ?
        "#,
        flavor_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            Row::from_row(&row)
                .context("Failed to parse flavor row")?
                .name,
        ),
        None => None,
    })
}

#[tracing::instrument(name = "select_flavor_name_from_db", skip(transaction))]
pub async fn select_flavor_name_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_id: u64,
) -> Result<String, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_name_from_db(transaction, flavor_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError(
            "User with given ID not found".to_string(),
        ))
}
