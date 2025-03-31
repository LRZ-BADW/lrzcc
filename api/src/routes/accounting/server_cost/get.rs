use std::collections::HashMap;

use actix_web::{
    web::{Data, Query, ReqData},
    HttpResponse,
};
use anyhow::{anyhow, Context};
use chrono::{DateTime, Datelike, TimeZone, Utc};
use indexmap::IndexMap;
use lrzcc_wire::{
    accounting::{
        ServerCostAll, ServerCostParams, ServerCostProject, ServerCostServer,
        ServerCostSimple, ServerCostUser,
    },
    pricing::FlavorPrice,
    user::{Project, User},
};
use serde::Serialize;
use sqlx::{MySql, MySqlPool, Transaction};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    authorization::require_admin_user,
    database::{
        accounting::server_state::select_user_class_by_server_from_db,
        pricing::flavor_price::select_flavor_prices_for_period_from_db,
        resources::flavor::select_all_flavors_from_db,
        user::{
            project::{
                select_all_projects_from_db,
                select_user_class_by_project_from_db,
            },
            user::select_user_class_by_user_from_db,
        },
    },
    error::{OptionApiError, UnexpectedOnlyError},
    routes::accounting::server_consumption::get::{
        calculate_server_consumption_for_all,
        calculate_server_consumption_for_project,
        calculate_server_consumption_for_server,
        calculate_server_consumption_for_user, ServerConsumptionForAll,
        ServerConsumptionForProject, ServerConsumptionForUser,
    },
};

#[derive(Hash, PartialEq, Eq, Clone, EnumIter, Debug)]
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
type PricePeriods = IndexMap<DateTime<Utc>, Prices>;

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
            .entry(price.flavor_name.clone())
            .or_insert(0.0) = price.unit_price;
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
                .entry(price.flavor_name.clone())
                .or_insert(0.0) = price.unit_price;
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

pub async fn calculate_server_cost_for_server_normal(
    transaction: &mut Transaction<'_, MySql>,
    server_uuid: &str,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<ServerCostSimple, UnexpectedOnlyError> {
    let mut cost = ServerCostSimple { total: 0.0 };
    let Some(user_class) = select_user_class_by_server_from_db(
        transaction,
        server_uuid.to_string(),
    )
    .await?
    .map(|u| UserClass::from_u32(u as u32))
    .map_or(Ok(None), |r| r.map(Some))?
    else {
        return Ok(cost);
    };
    let price_periods =
        get_flavor_price_periods(transaction, begin, end).await?;

    let mut end_times =
        price_periods.keys().skip(1).cloned().collect::<Vec<_>>();
    end_times.push(end);

    for ((start_time, prices), end_time) in price_periods.iter().zip(end_times)
    {
        let consumption = calculate_server_consumption_for_server(
            transaction,
            server_uuid,
            Some(*start_time),
            Some(end_time),
            None,
        )
        .await?;
        for (flavor_name, flavor_consumption) in consumption {
            if flavor_consumption <= 0. {
                continue;
            }
            let flavor_cost = calculate_flavor_consumption_cost(
                flavor_consumption,
                prices.clone(),
                user_class.clone(),
                flavor_name,
            );
            if flavor_cost <= 0. {
                continue;
            }
            cost.total += flavor_cost;
        }
    }

    Ok(cost)
}

// TODO: can we use macros to get rid of the code duplication here
pub async fn calculate_server_cost_for_server_detail(
    transaction: &mut Transaction<'_, MySql>,
    server_uuid: &str,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<ServerCostServer, UnexpectedOnlyError> {
    let mut cost = ServerCostServer {
        total: 0.0,
        flavors: HashMap::new(),
    };
    let Some(user_class) = select_user_class_by_server_from_db(
        transaction,
        server_uuid.to_string(),
    )
    .await?
    .map(|u| UserClass::from_u32(u as u32))
    .map_or(Ok(None), |r| r.map(Some))?
    else {
        return Ok(cost);
    };
    let price_periods =
        get_flavor_price_periods(transaction, begin, end).await?;

    let mut end_times =
        price_periods.keys().skip(1).cloned().collect::<Vec<_>>();
    end_times.push(end);

    for ((start_time, prices), end_time) in price_periods.iter().zip(end_times)
    {
        let consumption = calculate_server_consumption_for_server(
            transaction,
            server_uuid,
            Some(*start_time),
            Some(end_time),
            None,
        )
        .await?;
        for (flavor_name, flavor_consumption) in consumption {
            let flavor_cost = calculate_flavor_consumption_cost(
                flavor_consumption,
                prices.clone(),
                user_class.clone(),
                flavor_name.clone(),
            );
            *cost.flavors.entry(flavor_name).or_default() += flavor_cost;
            if flavor_cost <= 0. {
                continue;
            }
            cost.total += flavor_cost;
        }
    }

    Ok(cost)
}

pub async fn calculate_server_cost_for_server(
    transaction: &mut Transaction<'_, MySql>,
    server_uuid: &str,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
    detail: Option<bool>,
) -> Result<ServerCostForServer, UnexpectedOnlyError> {
    Ok(match detail {
        Some(true) => ServerCostForServer::Detail(
            calculate_server_cost_for_server_detail(
                transaction,
                server_uuid,
                begin,
                end,
            )
            .await?,
        ),
        _ => ServerCostForServer::Normal(
            calculate_server_cost_for_server_normal(
                transaction,
                server_uuid,
                begin,
                end,
            )
            .await?,
        ),
    })
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ServerCostForUser {
    Normal(ServerCostSimple),
    Detail(ServerCostUser),
}

// TODO: shouldn't this return not found, when the user doesn't exist?
pub async fn calculate_server_cost_for_user_normal(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<ServerCostSimple, UnexpectedOnlyError> {
    let mut cost = ServerCostSimple { total: 0.0 };
    let Some(user_class) =
        select_user_class_by_user_from_db(transaction, user_id)
            .await?
            .map(UserClass::from_u32)
            .map_or(Ok(None), |r| r.map(Some))?
    else {
        return Ok(cost);
    };
    let price_periods =
        get_flavor_price_periods(transaction, begin, end).await?;

    let mut end_times =
        price_periods.keys().skip(1).cloned().collect::<Vec<_>>();
    end_times.push(end);

    for ((start_time, prices), end_time) in price_periods.iter().zip(end_times)
    {
        let ServerConsumptionForUser::Normal(consumption) =
            calculate_server_consumption_for_user(
                transaction,
                user_id,
                Some(*start_time),
                Some(end_time),
                None,
            )
            .await?
        else {
            return Err(
                anyhow!("Unexpected ServerConsumptionForUser variant").into()
            );
        };
        for (flavor_name, flavor_consumption) in consumption {
            if flavor_consumption <= 0. {
                continue;
            }
            let flavor_cost = calculate_flavor_consumption_cost(
                flavor_consumption,
                prices.clone(),
                user_class.clone(),
                flavor_name,
            );
            cost.total += flavor_cost;
        }
    }

    Ok(cost)
}

// TODO: can we use macros to get rid of the code duplication here
pub async fn calculate_server_cost_for_user_detail(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<ServerCostUser, UnexpectedOnlyError> {
    let mut cost = ServerCostUser {
        total: 0.0,
        flavors: HashMap::new(),
        servers: HashMap::new(),
    };
    let Some(user_class) =
        select_user_class_by_user_from_db(transaction, user_id)
            .await?
            .map(UserClass::from_u32)
            .map_or(Ok(None), |r| r.map(Some))?
    else {
        return Ok(cost);
    };
    let price_periods =
        get_flavor_price_periods(transaction, begin, end).await?;

    let mut end_times =
        price_periods.keys().skip(1).cloned().collect::<Vec<_>>();
    end_times.push(end);

    for ((start_time, prices), end_time) in price_periods.iter().zip(end_times)
    {
        let ServerConsumptionForUser::Detail(consumption) =
            calculate_server_consumption_for_user(
                transaction,
                user_id,
                Some(*start_time),
                Some(end_time),
                Some(true),
            )
            .await?
        else {
            return Err(anyhow!(
                "Unexpected ServerConsumptionForUser variant."
            )
            .into());
        };
        for (server_uuid, server_consumption) in consumption.servers {
            let server_cost = cost
                .servers
                .entry(server_uuid.clone())
                .or_insert(ServerCostServer {
                    total: 0.0,
                    flavors: HashMap::new(),
                });
            for (flavor_name, flavor_consumption) in server_consumption {
                let flavor_cost = calculate_flavor_consumption_cost(
                    flavor_consumption,
                    prices.clone(),
                    user_class.clone(),
                    flavor_name.clone(),
                );
                *server_cost.flavors.entry(flavor_name.clone()).or_default() +=
                    flavor_cost;
                *cost.flavors.entry(flavor_name).or_default() += flavor_cost;
                if flavor_cost <= 0. {
                    continue;
                }
                server_cost.total += flavor_cost;
                cost.total += flavor_cost;
            }
        }
    }

    Ok(cost)
}

pub async fn calculate_server_cost_for_user(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
    detail: Option<bool>,
) -> Result<ServerCostForUser, UnexpectedOnlyError> {
    Ok(match detail {
        Some(true) => ServerCostForUser::Detail(
            calculate_server_cost_for_user_detail(
                transaction,
                user_id,
                begin,
                end,
            )
            .await?,
        ),
        _ => ServerCostForUser::Normal(
            calculate_server_cost_for_user_normal(
                transaction,
                user_id,
                begin,
                end,
            )
            .await?,
        ),
    })
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ServerCostForProject {
    Normal(ServerCostSimple),
    Detail(ServerCostProject),
}

pub async fn calculate_server_cost_for_project_normal(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<ServerCostSimple, UnexpectedOnlyError> {
    let mut cost = ServerCostSimple { total: 0.0 };
    let Some(user_class) =
        select_user_class_by_project_from_db(transaction, project_id)
            .await?
            .map(UserClass::from_u32)
            .map_or(Ok(None), |r| r.map(Some))?
    else {
        return Ok(cost);
    };
    let price_periods =
        get_flavor_price_periods(transaction, begin, end).await?;

    let mut end_times =
        price_periods.keys().skip(1).cloned().collect::<Vec<_>>();
    end_times.push(end);

    for ((start_time, prices), end_time) in price_periods.iter().zip(end_times)
    {
        let ServerConsumptionForProject::Normal(consumption) =
            calculate_server_consumption_for_project(
                transaction,
                project_id,
                Some(*start_time),
                Some(end_time),
                None,
            )
            .await?
        else {
            return Err(anyhow!(
                "Unexpected ServerConsumptionForProject variant"
            )
            .into());
        };
        for (flavor_name, flavor_consumption) in consumption {
            if flavor_consumption <= 0. {
                continue;
            }
            let flavor_cost = calculate_flavor_consumption_cost(
                flavor_consumption,
                prices.clone(),
                user_class.clone(),
                flavor_name,
            );
            if flavor_cost <= 0. {
                continue;
            }
            cost.total += flavor_cost;
        }
    }

    Ok(cost)
}

// TODO: can we use macros to get rid of the code duplication here
pub async fn calculate_server_cost_for_project_detail(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<ServerCostProject, UnexpectedOnlyError> {
    let mut cost = ServerCostProject {
        total: 0.0,
        flavors: HashMap::new(),
        users: HashMap::new(),
    };
    let Some(user_class) =
        select_user_class_by_project_from_db(transaction, project_id)
            .await?
            .map(UserClass::from_u32)
            .map_or(Ok(None), |r| r.map(Some))?
    else {
        return Ok(cost);
    };
    let price_periods =
        get_flavor_price_periods(transaction, begin, end).await?;

    let mut end_times =
        price_periods.keys().skip(1).cloned().collect::<Vec<_>>();
    end_times.push(end);

    for ((start_time, prices), end_time) in price_periods.iter().zip(end_times)
    {
        let ServerConsumptionForProject::Detail(consumption) =
            calculate_server_consumption_for_project(
                transaction,
                project_id,
                Some(*start_time),
                Some(end_time),
                Some(true),
            )
            .await?
        else {
            return Err(anyhow!(
                "Unexpected ServerConsumptionForProject variant"
            )
            .into());
        };
        for (user_name, user_consumption) in consumption.users {
            let user_cost =
                cost.users
                    .entry(user_name.clone())
                    .or_insert(ServerCostUser {
                        total: 0.0,
                        flavors: HashMap::new(),
                        servers: HashMap::new(),
                    });
            for (server_uuid, server_consumption) in user_consumption.servers {
                let server_cost = user_cost
                    .servers
                    .entry(server_uuid.clone())
                    .or_insert(ServerCostServer {
                        total: 0.0,
                        flavors: HashMap::new(),
                    });
                for (flavor_name, flavor_consumption) in server_consumption {
                    let flavor_cost = calculate_flavor_consumption_cost(
                        flavor_consumption,
                        prices.clone(),
                        user_class.clone(),
                        flavor_name.clone(),
                    );
                    *server_cost
                        .flavors
                        .entry(flavor_name.clone())
                        .or_default() += flavor_cost;
                    *user_cost
                        .flavors
                        .entry(flavor_name.clone())
                        .or_default() += flavor_cost;
                    *cost.flavors.entry(flavor_name).or_default() +=
                        flavor_cost;
                    if flavor_cost <= 0. {
                        continue;
                    }
                    server_cost.total += flavor_cost;
                    user_cost.total += flavor_cost;
                    cost.total += flavor_cost;
                }
            }
        }
    }

    Ok(cost)
}

pub async fn calculate_server_cost_for_project(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
    detail: Option<bool>,
) -> Result<ServerCostForProject, UnexpectedOnlyError> {
    Ok(match detail {
        Some(true) => ServerCostForProject::Detail(
            calculate_server_cost_for_project_detail(
                transaction,
                project_id,
                begin,
                end,
            )
            .await?,
        ),
        _ => ServerCostForProject::Normal(
            calculate_server_cost_for_project_normal(
                transaction,
                project_id,
                begin,
                end,
            )
            .await?,
        ),
    })
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ServerCostForAll {
    Normal(ServerCostSimple),
    Detail(ServerCostAll),
}

// TODO: optimize/parallelize this and other functions
pub async fn calculate_server_cost_for_all_normal(
    transaction: &mut Transaction<'_, MySql>,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<ServerCostSimple, UnexpectedOnlyError> {
    let mut cost = ServerCostSimple { total: 0.0 };
    let price_periods =
        get_flavor_price_periods(transaction, begin, end).await?;

    let mut end_times =
        price_periods.keys().skip(1).cloned().collect::<Vec<_>>();
    end_times.push(end);

    let projects = select_all_projects_from_db(transaction)
        .await?
        .into_iter()
        .map(|p| (p.name.clone(), p))
        .collect::<HashMap<_, _>>();

    for ((start_time, prices), end_time) in price_periods.iter().zip(end_times)
    {
        let ServerConsumptionForAll::Detail(consumption) =
            calculate_server_consumption_for_all(
                transaction,
                Some(*start_time),
                Some(end_time),
                Some(true),
            )
            .await?
        else {
            return Err(
                anyhow!("Unexpected ServerConsumptionForAll variant").into()
            );
        };
        for (project_name, project_consumption) in consumption.projects {
            let Some(project) = projects.get(&project_name) else {
                continue;
            };
            let Ok(user_class) = UserClass::from_u32(project.user_class) else {
                continue;
            };

            for (flavor_name, flavor_consumption) in project_consumption.total {
                if flavor_consumption == 0. {
                    continue;
                }
                let flavor_cost = calculate_flavor_consumption_cost(
                    flavor_consumption,
                    prices.clone(),
                    user_class.clone(),
                    flavor_name,
                );
                if flavor_cost <= 0. {
                    continue;
                }
                cost.total += flavor_cost;
            }
        }
    }

    Ok(cost)
}

// TODO: can we use macros to get rid of the code duplication here
pub async fn calculate_server_cost_for_all_detail(
    transaction: &mut Transaction<'_, MySql>,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<ServerCostAll, UnexpectedOnlyError> {
    let mut cost = ServerCostAll {
        total: 0.0,
        flavors: HashMap::new(),
        projects: HashMap::new(),
    };
    let price_periods =
        get_flavor_price_periods(transaction, begin, end).await?;

    let mut end_times =
        price_periods.keys().skip(1).cloned().collect::<Vec<_>>();
    end_times.push(end);

    let projects = select_all_projects_from_db(transaction)
        .await?
        .into_iter()
        .map(|p| (p.name.clone(), p))
        .collect::<HashMap<_, _>>();

    for ((start_time, prices), end_time) in price_periods.iter().zip(end_times)
    {
        let ServerConsumptionForAll::Detail(consumption) =
            calculate_server_consumption_for_all(
                transaction,
                Some(*start_time),
                Some(end_time),
                Some(true),
            )
            .await?
        else {
            return Err(
                anyhow!("Unexpected ServerConsumptionForAll variant").into()
            );
        };
        for (project_name, project_consumption) in consumption.projects {
            let Some(project) = projects.get(&project_name) else {
                continue;
            };
            let Ok(user_class) = UserClass::from_u32(project.user_class) else {
                continue;
            };
            let project_cost = cost
                .projects
                .entry(project_name.clone())
                .or_insert(ServerCostProject {
                    total: 0.0,
                    flavors: HashMap::new(),
                    users: HashMap::new(),
                });

            for (user_name, user_consumption) in project_consumption.users {
                let user_cost = project_cost
                    .users
                    .entry(user_name.clone())
                    .or_insert(ServerCostUser {
                        total: 0.0,
                        flavors: HashMap::new(),
                        servers: HashMap::new(),
                    });
                for (server_uuid, server_consumption) in
                    user_consumption.servers
                {
                    let server_cost = user_cost
                        .servers
                        .entry(server_uuid.clone())
                        .or_insert(ServerCostServer {
                            total: 0.0,
                            flavors: HashMap::new(),
                        });
                    for (flavor_name, flavor_consumption) in server_consumption
                    {
                        let flavor_cost = calculate_flavor_consumption_cost(
                            flavor_consumption,
                            prices.clone(),
                            user_class.clone(),
                            flavor_name.clone(),
                        );
                        *server_cost
                            .flavors
                            .entry(flavor_name.clone())
                            .or_default() += flavor_cost;
                        *user_cost
                            .flavors
                            .entry(flavor_name.clone())
                            .or_default() += flavor_cost;
                        *project_cost
                            .flavors
                            .entry(flavor_name.clone())
                            .or_default() += flavor_cost;
                        *cost.flavors.entry(flavor_name).or_default() +=
                            flavor_cost;
                        if flavor_cost <= 0. {
                            continue;
                        }
                        server_cost.total += flavor_cost;
                        user_cost.total += flavor_cost;
                        project_cost.total += flavor_cost;
                        cost.total += flavor_cost;
                    }
                }
            }
        }
    }

    Ok(cost)
}

pub async fn calculate_server_cost_for_all(
    transaction: &mut Transaction<'_, MySql>,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
    detail: Option<bool>,
) -> Result<ServerCostForAll, UnexpectedOnlyError> {
    Ok(match detail {
        Some(true) => ServerCostForAll::Detail(
            calculate_server_cost_for_all_detail(transaction, begin, end)
                .await?,
        ),
        _ => ServerCostForAll::Normal(
            calculate_server_cost_for_all_normal(transaction, begin, end)
                .await?,
        ),
    })
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ServerCost {
    Server(ServerCostForServer),
    User(ServerCostForUser),
    Project(ServerCostForProject),
    All(ServerCostForAll),
}

#[tracing::instrument(name = "server_cost")]
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
                begin.into(),
                end.into(),
                params.detail,
            )
            .await?,
        )
    } else if let Some(project_id) = params.project {
        ServerCost::Project(
            calculate_server_cost_for_project(
                &mut transaction,
                project_id as u64,
                begin.into(),
                end.into(),
                params.detail,
            )
            .await?,
        )
    } else if let Some(user_id) = params.user {
        ServerCost::User(
            calculate_server_cost_for_user(
                &mut transaction,
                user_id as u64,
                begin.into(),
                end.into(),
                params.detail,
            )
            .await?,
        )
    } else if let Some(server_id) = params.server.clone() {
        ServerCost::Server(
            calculate_server_cost_for_server(
                &mut transaction,
                server_id.as_str(),
                begin.into(),
                end.into(),
                params.detail,
            )
            .await?,
        )
    } else {
        ServerCost::User(
            calculate_server_cost_for_user(
                &mut transaction,
                user.id as u64,
                begin.into(),
                end.into(),
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
