use crate::authorization::require_admin_user;
use crate::database::pricing::flavor_price::select_flavor_prices_for_period_from_db;
use crate::error::{OptionApiError, UnexpectedOnlyError};
use actix_web::web::{Data, Query, ReqData};
use actix_web::HttpResponse;
use anyhow::{anyhow, Context};
use chrono::{DateTime, Datelike, TimeZone, Utc};
use lrzcc_wire::accounting::{
    ServerCostAll, ServerCostParams, ServerCostProject, ServerCostServer,
    ServerCostSimple, ServerCostUser,
};
use lrzcc_wire::pricing::FlavorPrice;
use lrzcc_wire::user::{Project, User};
use serde::Serialize;
use sqlx::{MySql, MySqlPool, Transaction};
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone)]
enum UserClass {
    UC1 = 1,
    UC2 = 2,
    UC3 = 3,
    UC4 = 4,
    UC5 = 5,
    UC6 = 6,
}

impl UserClass {
    fn from_u32(value: u32) -> Result<Self, UnexpectedOnlyError> {
        match value {
            1 => Ok(UserClass::UC1),
            2 => Ok(UserClass::UC2),
            3 => Ok(UserClass::UC3),
            4 => Ok(UserClass::UC4),
            5 => Ok(UserClass::UC5),
            6 => Ok(UserClass::UC6),
            _ => Err(anyhow!("Got non-existing user-class.").into()),
        }
    }
}

type PricesForPeriod = HashMap<UserClass, HashMap<String, Vec<FlavorPrice>>>;

async fn get_flavor_prices_for_period(
    transaction: &mut Transaction<'_, MySql>,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<PricesForPeriod, UnexpectedOnlyError> {
    let price_list =
        select_flavor_prices_for_period_from_db(transaction, begin, end)
            .await?;
    let mut prices = HashMap::new();
    for price in price_list {
        prices
            .entry(UserClass::from_u32(price.user_class)?)
            .or_insert_with(HashMap::new)
            .entry(price.flavor_name.clone())
            .or_insert_with(Vec::new)
            .push(price);
    }
    for uprices in prices.values_mut() {
        for fprices in uprices.values_mut() {
            let mut i = fprices.len() - 1;
            while i > 0 {
                if fprices[i].start_time <= begin {
                    *fprices = fprices.split_off(i);
                    break;
                }
                i -= 1;
            }
        }
    }
    Ok(prices)
}

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
