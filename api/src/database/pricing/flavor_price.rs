use crate::error::{NotFoundOrUnexpectedApiError, UnexpectedOnlyError};
use anyhow::Context;
use chrono::{DateTime, Utc};
use lrzcc_wire::pricing::FlavorPrice;
use sqlx::{Executor, FromRow, MySql, Transaction};

#[tracing::instrument(
    name = "select_maybe_flavor_price_from_db",
    skip(transaction)
)]
pub async fn select_maybe_flavor_price_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_price_id: u64,
) -> Result<Option<FlavorPrice>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    pub struct Row {
        pub id: u32,
        pub flavor: u32,
        pub flavor_name: String,
        pub user_class: u32,
        pub unit_price: f64,
        pub start_time: DateTime<Utc>,
    }
    let query = sqlx::query!(
        r#"
        SELECT
            p.id,
            p.flavor_id as flavor,
            f.name as flavor_name, 
            p.user_class as user_class,
            p.unit_price as unit_price,
            p.start_time as start_time
        FROM
            pricing_flavorprice as p,
            resources_flavor as f
        WHERE
            p.flavor_id = f.id AND
            p.id = ?
        "#,
        flavor_price_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => {
            let row = Row::from_row(&row)
                .context("Failed to parse flavor price row")?;
            Some(FlavorPrice {
                id: row.id,
                flavor: row.flavor,
                flavor_name: row.flavor_name,
                user_class: row.user_class,
                unit_price: row.unit_price,
                start_time: row.start_time.fixed_offset(),
            })
        }
        None => None,
    })
}

#[tracing::instrument(name = "select_flavor_price_from_db", skip(transaction))]
pub async fn select_flavor_price_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_price_id: u64,
) -> Result<FlavorPrice, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_price_from_db(transaction, flavor_price_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError(
            "Flavor price with given ID not found".to_string(),
        ))
}