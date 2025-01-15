use crate::authorization::require_admin_user;
use crate::error::{OptionApiError, UnexpectedOnlyError};
use actix_web::web::{Data, Query, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use chrono::{DateTime, Datelike, TimeZone, Utc};
use lrzcc_wire::accounting::{
    ServerCostAll, ServerCostParams, ServerCostProject, ServerCostServer,
    ServerCostSimple, ServerCostUser,
};
use lrzcc_wire::user::{Project, User};
use serde::Serialize;
use sqlx::{MySql, MySqlPool, Transaction};

#[derive(Serialize)]
#[serde(untagged)]
pub enum ServerCostForServer {
    Normal(ServerCostSimple),
    Detail(ServerCostServer),
}

pub async fn calculate_server_cost_for_server(
    transaction: &mut Transaction<'_, MySql>,
    server_uuid: &str,
    begin: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    detail: Option<bool>,
) -> Result<ServerCostForServer, UnexpectedOnlyError> {
    todo!()
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ServerCostForUser {
    Normal(ServerCostSimple),
    Detail(ServerCostUser),
}

pub async fn calculate_server_cost_for_user(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    begin: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    detail: Option<bool>,
) -> Result<ServerCostForUser, UnexpectedOnlyError> {
    todo!()
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ServerCostForProject {
    Normal(ServerCostSimple),
    Detail(ServerCostProject),
}

pub async fn calculate_server_cost_for_project(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
    begin: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    detail: Option<bool>,
) -> Result<ServerCostForProject, UnexpectedOnlyError> {
    todo!()
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ServerCostForAll {
    Normal(ServerCostSimple),
    Detail(ServerCostAll),
}

pub async fn calculate_server_cost_for_all(
    transaction: &mut Transaction<'_, MySql>,
    begin: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    detail: Option<bool>,
) -> Result<ServerCostForAll, UnexpectedOnlyError> {
    todo!()
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ServerCost {
    Server(ServerCostForServer),
    User(ServerCostForUser),
    Project(ServerCostForProject),
    All(ServerCostForAll),
}

#[tracing::instrument(name = "server_consumption")]
pub async fn server_cost(
    user: ReqData<User>,
    // TODO: not necessary?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<ServerCostParams>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    // TODO: add proper permission check
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
    let cost = if params.all.unwrap_or(false) {
        ServerCost::All(
            calculate_server_cost_for_all(
                &mut transaction,
                Some(begin.into()),
                Some(end.into()),
                params.detail,
            )
            .await?,
        )
    } else if let Some(project_id) = params.project {
        ServerCost::Project(
            calculate_server_cost_for_project(
                &mut transaction,
                project_id as u64,
                Some(begin.into()),
                Some(end.into()),
                params.detail,
            )
            .await?,
        )
    } else if let Some(user_id) = params.user {
        ServerCost::User(
            calculate_server_cost_for_user(
                &mut transaction,
                user_id as u64,
                Some(begin.into()),
                Some(end.into()),
                params.detail,
            )
            .await?,
        )
    } else if let Some(server_id) = params.server.clone() {
        ServerCost::Server(
            calculate_server_cost_for_server(
                &mut transaction,
                server_id.as_str(),
                Some(begin.into()),
                Some(end.into()),
                None,
            )
            .await?,
        )
    } else {
        ServerCost::User(
            calculate_server_cost_for_user(
                &mut transaction,
                user.id as u64,
                Some(begin.into()),
                Some(end.into()),
                params.detail,
            )
            .await?,
        )
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(cost))
}
