use crate::error::{NotFoundOrUnexpectedApiError, UnexpectedOnlyError};
use anyhow::Context;
use lrzcc_wire::resources::FlavorGroup;
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

#[derive(Clone, Debug, PartialEq, FromRow)]
pub struct FlavorGroupDb {
    pub id: u32,
    pub name: String,
    pub project_id: u32,
}

#[tracing::instrument(
    name = "select_maybe_flavor_group_from_db",
    skip(transaction)
)]
pub async fn select_maybe_flavor_group_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_group_id: u64,
) -> Result<Option<FlavorGroup>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    pub struct FlavorGroupDb {
        pub id: u32,
        pub name: String,
        pub project_id: u32,
    }
    let query1 = sqlx::query!(
        r#"
        SELECT id, name, project_id
        FROM resources_flavorgroup
        WHERE id = ?
        "#,
        flavor_group_id
    );
    let row1 = transaction
        .fetch_optional(query1)
        .await
        .context("Failed to execute select query")?;
    let flavor_group = match row1 {
        Some(row) => FlavorGroupDb::from_row(&row)
            .context("Failed to parse flavor group row")?,
        None => return Ok(None),
    };
    #[derive(FromRow)]
    pub struct FlavorIdDb {
        pub id: u32,
    }
    let query2 = sqlx::query!(
        r#"
        SELECT id
        FROM resources_flavor
        WHERE group_id = ?
        "#,
        flavor_group_id
    );
    let flavors = transaction
        .fetch_all(query2)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|row| FlavorIdDb::from_row(&row))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to parse flavor row")?
        .into_iter()
        .map(|row| row.id)
        .collect::<Vec<_>>();
    Ok(Some(FlavorGroup {
        id: flavor_group.id,
        name: flavor_group.name,
        project: flavor_group.project_id,
        flavors,
    }))
}

#[tracing::instrument(name = "select_flavor_group_from_db", skip(transaction))]
pub async fn select_flavor_group_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_group_id: u64,
) -> Result<FlavorGroup, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_group_from_db(transaction, flavor_group_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError(
            "Flavor group with given ID not found".to_string(),
        ))
}
