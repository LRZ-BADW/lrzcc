use crate::authorization::require_admin_user;
use crate::database::pricing::flavor_price::select_flavor_prices_for_period_from_db;
use crate::database::resources::flavor::select_all_flavors_from_db;
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
use strum::{EnumIter, IntoEnumIterator};

#[derive(Hash, PartialEq, Eq, Clone, EnumIter)]
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

async fn get_flavor_price_map_for_period(
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
            // TODO: .default() should work here, too
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

async fn get_flavor_prices_for_period(
    transaction: &mut Transaction<'_, MySql>,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<FlavorPrice>, UnexpectedOnlyError> {
    let mut prices = get_flavor_price_map_for_period(transaction, begin, end)
        .await?
        .into_iter()
        .flat_map(|(_, v)| v.into_iter().flat_map(|(_, w)| w))
        .collect::<Vec<FlavorPrice>>();
    prices.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());
    Ok(prices)
}

type Prices = HashMap<UserClass, HashMap<String, f64>>;
type PricePeriods = HashMap<DateTime<Utc>, Prices>;

async fn get_flavor_price_periods(
    transaction: &mut Transaction<'_, MySql>,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<PricePeriods, UnexpectedOnlyError> {
    let flavors = select_all_flavors_from_db(transaction).await?;
    let mut current_prices = Prices::new();
    for user_class in UserClass::iter() {
        for flavor in flavors.clone() {
            current_prices
                .entry(user_class.clone())
                .or_default()
                .entry(flavor.name.clone())
                .or_insert(0.0);
        }
    }

    let prices = get_flavor_prices_for_period(transaction, begin, end).await?;

    let mut i = 0;
    while i < prices.len() {
        let price = prices.get(i).unwrap();
        if price.start_time > begin {
            break;
        }
        *current_prices
            .get_mut(&UserClass::from_u32(price.user_class)?)
            .unwrap()
            .get_mut(&price.flavor_name)
            .unwrap() = price.unit_price;
        i += 1;
    }

    let mut periods = PricePeriods::new();

    let mut current_time = begin;
    periods.insert(current_time, current_prices.clone());

    if i == prices.len() {
        return Ok(periods);
    }

    current_time = prices.get(i).unwrap().start_time.to_utc();
    while i < prices.len() {
        let price = prices.get(i).unwrap();
        if price.start_time.to_utc() == current_time {
            *current_prices
                .get_mut(&UserClass::from_u32(price.user_class)?)
                .unwrap()
                .get_mut(&price.flavor_name)
                .unwrap() = price.unit_price;
        } else {
            periods.insert(current_time, current_prices.clone());
            current_time = prices.get(i).unwrap().start_time.to_utc();
        }
        i += 1;
    }
    periods.insert(current_time, current_prices.clone());

    Ok(periods)
}

fn calculate_flavor_consumption_cost(
    flavor_consumption: f64,
    prices: Prices,
    user_class: UserClass,
    flavor: String,
) -> f64 {
    let mut cost = 0.0;
    if let Some(price) = prices.get(&user_class).unwrap().get(&flavor) {
        cost = (flavor_consumption * price) / ((365 * 24 * 60 * 60) as f64);
    }
    cost
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
