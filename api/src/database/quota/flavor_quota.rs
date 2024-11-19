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

#[tracing::instrument(
    name = "select_all_flavor_quotas_from_db",
    skip(transaction)
)]
pub async fn select_all_flavor_quotas_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<FlavorQuota>, UnexpectedOnlyError> {
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
            q.user_id = u.id
        "#,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| FlavorQuota::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor quota")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_flavor_quotas_by_flavor_group_from_db",
    skip(transaction)
)]
pub async fn select_flavor_quotas_by_flavor_group_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_group_id: u64,
) -> Result<Vec<FlavorQuota>, UnexpectedOnlyError> {
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
            g.id = ?
        "#,
        flavor_group_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| FlavorQuota::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor quota")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_flavor_quotas_by_user_from_db",
    skip(transaction)
)]
pub async fn select_flavor_quotas_by_user_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<Vec<FlavorQuota>, UnexpectedOnlyError> {
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
            u.id = ?
        "#,
        user_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| FlavorQuota::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor quota")?;
    Ok(rows)
}
