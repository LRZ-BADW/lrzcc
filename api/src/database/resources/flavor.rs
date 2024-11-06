use crate::error::{NotFoundOrUnexpectedApiError, UnexpectedOnlyError};
use anyhow::Context;
use lrzcc_wire::resources::{Flavor, FlavorMinimal};
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

#[tracing::instrument(name = "select_maybe_flavor_from_db", skip(transaction))]
pub async fn select_maybe_flavor_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_id: u64,
) -> Result<Option<Flavor>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT f.id, f.name, f.openstack_id, f.weight, f.group_id, g.name as group_name
        FROM resources_flavor as f, resources_flavorgroup as g
        WHERE
            f.group_id = g.id AND
            f.id = ?
        "#,
        flavor_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    // TODO: isn't there a nicer way to write this?
    Ok(match row {
        Some(row) => {
            Some(Flavor::from_row(&row).context("Failed to parse flavor row")?)
        }
        None => None,
    })
}

#[tracing::instrument(name = "select_flavor_from_db", skip(transaction))]
pub async fn select_flavor_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_id: u64,
) -> Result<Flavor, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_from_db(transaction, flavor_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError(
            "Flavor  with given ID not found".to_string(),
        ))
}

#[tracing::instrument(
    name = "select_minimal_flavors_by_group_from_db",
    skip(transaction)
)]
pub async fn select_minimal_flavors_by_group_from_db(
    transaction: &mut Transaction<'_, MySql>,
    group_id: u64,
) -> Result<Vec<FlavorMinimal>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT f.id, f.name
        FROM resources_flavor as f
        WHERE f.group_id = ?
        "#,
        group_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| FlavorMinimal::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor")?;
    Ok(rows)
}
