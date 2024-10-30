use crate::error::{NotFoundOrUnexpectedApiError, UnexpectedOnlyError};
use anyhow::Context;
use chrono::{DateTime, Utc};
use lrzcc_wire::accounting::ServerState;
use sqlx::{Executor, FromRow, MySql, Transaction};

#[tracing::instrument(
    name = "select_maybe_server_state_from_db",
    skip(transaction)
)]
pub async fn select_maybe_server_state_from_db(
    transaction: &mut Transaction<'_, MySql>,
    server_state_id: u64,
) -> Result<Option<ServerState>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    pub struct Row {
        pub id: u32,
        pub begin: DateTime<Utc>,
        pub end: Option<DateTime<Utc>>,
        pub instance_id: String,
        pub instance_name: String,
        pub flavor: u32,
        pub flavor_name: String,
        pub status: String,
        pub user: u32,
        pub username: String,
    }
    let query = sqlx::query!(
        r#"
        SELECT
            s.id as id,
            s.begin as begin,
            s.end as end,
            ss.instance_id as instance_id,
            ss.instance_name as instance_name,
            f.id as flavor,
            f.name as flavor_name,
            ss.status as status,
            u.id as user,
            u.name as username
        FROM
            accounting_state as s,
            accounting_serverstate as ss,
            resources_flavor as f,
            user_user as u
        WHERE
            ss.flavor_id = f.id AND
            ss.user_id = u.id AND
            ss.state_ptr_id = s.id AND
            s.id = ?
        "#,
        server_state_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => {
            let row = Row::from_row(&row)
                .context("Failed to parse flavor price row")?;
            Some(ServerState {
                id: row.id,
                begin: row.begin.fixed_offset(),
                end: row.end.map(|end| end.fixed_offset()),
                instance_id: row.instance_id,
                instance_name: row.instance_name,
                flavor: row.flavor,
                flavor_name: row.flavor_name,
                status: row.status,
                user: row.user,
                username: row.username,
            })
        }
        None => None,
    })
}

#[tracing::instrument(name = "select_server_state_from_db", skip(transaction))]
pub async fn select_server_state_from_db(
    transaction: &mut Transaction<'_, MySql>,
    server_state_id: u64,
) -> Result<ServerState, NotFoundOrUnexpectedApiError> {
    select_maybe_server_state_from_db(transaction, server_state_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError(
            "Server state with given ID not found".to_string(),
        ))
}
