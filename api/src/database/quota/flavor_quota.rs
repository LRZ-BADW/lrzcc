use crate::error::{NotFoundOrUnexpectedApiError, UnexpectedOnlyError};
use anyhow::Context;
use lrzcc_wire::quota::FlavorQuota;
use sqlx::{Executor, FromRow, MySql, Transaction};

#[tracing::instrument(
    name = "select_maybe_flavor_quota_from_db",
    skip(transaction)
)]
pub async fn select_maybe_flavor_quota_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_quota_id: u64,
) -> Result<Option<FlavorQuota>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            q.id as id,
            u.id as user,
            u.name as username,
            q.quota as quota,
            g.id as flavor_group,
            g.name as flavor_group_name
        FROM
            quota_flavorquota as f,
            quota_quota as q,
            resources_flavorgroup as g,
            user_user as u
        WHERE
            f.quota_ptr_id = q.id AND
            f.flavor_group_id = g.id AND
            q.user_id = u.id AND
            q.id = ?
        "#,
        flavor_quota_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            FlavorQuota::from_row(&row)
                .context("Failed to parse flavor quota row")?,
        ),
        None => None,
    })
}

#[tracing::instrument(name = "select_flavor_quota_from_db", skip(transaction))]
pub async fn select_flavor_quota_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_quota_id: u64,
) -> Result<FlavorQuota, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_quota_from_db(transaction, flavor_quota_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError(
            "Flavor quota with given ID not found".to_string(),
        ))
}
