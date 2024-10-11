use crate::error::{NotFoundOrUnexpectedApiError, UnexpectedOnlyError};
use anyhow::Context;
use sqlx::{Executor, FromRow, MySql, Transaction};

#[tracing::instrument(
    name = "select_maybe_flavor_group_name_from_db",
    skip(transaction)
)]
pub async fn select_maybe_flavor_group_name_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_group_id: u64,
) -> Result<Option<String>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    #[allow(dead_code)]
    struct Row {
        name: String,
    }
    let query = sqlx::query!(
        r#"
        SELECT name
        FROM resources_flavorgroup
        WHERE id = ?
        "#,
        flavor_group_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            Row::from_row(&row)
                .context("Failed to parse flavor group row")?
                .name,
        ),
        None => None,
    })
}

#[tracing::instrument(
    name = "select_flavor_group_name_from_db",
    skip(transaction)
)]
pub async fn select_flavor_group_name_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_group_id: u64,
) -> Result<String, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_group_name_from_db(transaction, flavor_group_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError(
            "Flavor group with given ID not found".to_string(),
        ))
}
