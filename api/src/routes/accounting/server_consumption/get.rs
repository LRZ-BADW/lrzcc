use crate::authorization::require_admin_user;
use crate::database::accounting::server_state::select_ordered_server_states_by_server_begin_and_end_from_db;
use crate::error::{OptionApiError, UnexpectedOnlyError};
use actix_web::web::{Data, Query, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use chrono::{DateTime, Datelike, TimeZone, Utc};
use lrzcc_wire::accounting::{
    ServerConsumptionParams, ServerConsumptionServer, ServerState,
};
use lrzcc_wire::user::{Project, User};
use sqlx::{MySql, MySqlPool, Transaction};

const CONSUMING_STATES: [&str; 15] = [
    "ACTIVE",
    "BUILD",
    "HARD_REBOOT",
    "MIGRATING",
    "PASSWORD",
    "PAUSED",
    "REBOOT",
    "REBUILD",
    "RESCUE",
    "RESIZE",
    "REVERT_RESIZE",
    "SHUTOFF",
    "SUSPENDED",
    "UNKNOWN",
    "VERIFY_RESIZE",
];

pub async fn calculate_server_consumption_for_server(
    transaction: &mut Transaction<'_, MySql>,
    server_uuid: &str,
    begin: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    states: Option<Vec<ServerState>>,
) -> Result<ServerConsumptionServer, UnexpectedOnlyError> {
    let mut states = match states {
        Some(states) => states,
        None => {
            select_ordered_server_states_by_server_begin_and_end_from_db(
                transaction,
                server_uuid.to_string(),
                begin,
                end,
            )
            .await?
        }
    };
    let mut consumption = ServerConsumptionServer::default();
    if states.is_empty() {
        return Ok(consumption);
    }
    let first = states.first_mut().unwrap();
    if let Some(begin) = begin {
        if begin.fixed_offset() > first.begin {
            first.begin = begin.fixed_offset();
        }
    }
    let last = states.last_mut().unwrap();
    if last.end.is_none() {
        if let Some(end) = end {
            last.end = Some(end.fixed_offset());
        }
    }
    if let Some(end) = end {
        if end.fixed_offset() < last.end.unwrap() {
            last.end = Some(end.fixed_offset());
        }
    }
    for state in states {
        if !consumption.contains_key(&state.flavor_name) {
            consumption.insert(state.flavor_name.clone(), 0.0);
        }
        if !CONSUMING_STATES.contains(&state.status.as_str()) {
            continue;
        }
        *consumption.get_mut(&state.flavor_name).unwrap() +=
            (state.end.unwrap() - state.begin).num_seconds() as f64;
    }
    // TODO:
    Ok(consumption)
}

#[tracing::instrument(name = "server_consumption")]
pub async fn server_consumption(
    user: ReqData<User>,
    // TODO: not necessary?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<ServerConsumptionParams>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let end = params.end.unwrap_or(Utc::now().fixed_offset());
    let begin = params.begin.unwrap_or(
        Utc.with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 0, 0)
            .unwrap()
            .fixed_offset(),
    );
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let consumption = calculate_server_consumption_for_server(
        &mut transaction,
        params.server.clone().unwrap().as_str(),
        Some(begin.into()),
        Some(end.into()),
        None,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(consumption))
}
