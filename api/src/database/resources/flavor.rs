use crate::error::{NotFoundOrUnexpectedApiError, UnexpectedOnlyError};
use anyhow::Context;
use lrzcc_wire::resources::{
    Flavor, FlavorDetailed, FlavorGroupMinimal, FlavorMinimal,
};
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

#[tracing::instrument(
    name = "select_maybe_user_detail_from_db",
    skip(transaction)
)]
pub async fn select_maybe_flavor_detail_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_id: u64,
) -> Result<Option<FlavorDetailed>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    pub struct FlavorDb {
        pub id: u32,
        pub name: String,
        pub openstack_id: String, // UUIDv4
        pub group_id: Option<u32>,
        pub group_name: Option<String>,
        pub weight: u32,
    }
    let query = sqlx::query!(
        r#"
        SELECT
            f.id AS id,
            f.name AS name,
            f.openstack_id AS openstack_id,
            g.id AS group_id,
            g.name AS group_name,
            f.weight AS weight
        FROM resources_flavor AS f, resources_flavorgroup AS g
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
    let flavor = match row {
        Some(row) => {
            FlavorDb::from_row(&row).context("Failed to parse flavor row")?
        }
        None => return Ok(None),
    };
    Ok(Some(FlavorDetailed {
        id: flavor.id,
        name: flavor.name,
        openstack_id: flavor.openstack_id,
        group: match (flavor.group_id, flavor.group_name.clone()) {
            (Some(id), Some(name)) => Some(FlavorGroupMinimal { id, name }),
            _ => None,
        },
        group_name: flavor.group_name,
        weight: flavor.weight,
    }))
}

#[tracing::instrument(name = "select_user_detail_from_db", skip(transaction))]
pub async fn select_flavor_detail_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<FlavorDetailed, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_detail_from_db(transaction, user_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError(
            "Flavor with given ID or linked project not found".to_string(),
        ))
}
