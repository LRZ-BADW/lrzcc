use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use anyhow::{Context, anyhow};
use avina_wire::{
    budgeting::{
        UserBudgetOverCombined, UserBudgetOverCombinedDetail,
        UserBudgetOverDetail, UserBudgetOverParams, UserBudgetOverSimple,
    },
    user::User,
};
use chrono::{DateTime, Datelike, Utc};
use serde::Serialize;
use sqlx::{MySql, MySqlPool, Transaction};

use crate::{
    authorization::{
        require_admin_user, require_master_user_or_return_not_found,
        require_user_or_project_master_or_not_found,
    },
    database::{
        budgeting::{
            project_budget::select_maybe_project_budget_by_project_and_year_from_db,
            user_budget::{
                select_maybe_user_budget_by_user_and_year_from_db,
                select_maybe_user_budget_from_db, select_user_budget_from_db,
                select_user_budgets_by_project_and_year_from_db,
                select_user_budgets_by_year_from_db,
            },
        },
        user::user::select_user_from_db,
    },
    error::{OptionApiError, UnexpectedOnlyError},
    routes::{
        accounting::server_cost::get::{
            ServerCostForUser, calculate_server_cost_for_user,
        },
        server_cost::get::{
            ServerCostForProject, calculate_server_cost_for_project,
        },
    },
    utils::start_of_the_year,
};

#[derive(Serialize)]
#[serde(untagged)]
pub enum UserBudgetOver {
    Normal(Vec<UserBudgetOverSimple>),
    Combined(Vec<UserBudgetOverCombined>),
    Detail(Vec<UserBudgetOverDetail>),
    CombinedDetail(Vec<UserBudgetOverCombinedDetail>),
}

pub async fn calculate_user_budget_over_for_budget_normal(
    transaction: &mut Transaction<'_, MySql>,
    budget_id: u64,
    end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverSimple>, UnexpectedOnlyError> {
    let mut overs = vec![];
    let Some(budget) =
        select_maybe_user_budget_from_db(transaction, budget_id).await?
    else {
        return Ok(overs);
    };
    let year = budget.year;
    if year != end.year() as u32 {
        return Ok(overs);
    }
    let begin = start_of_the_year(year);
    let ServerCostForUser::Normal(cost) = calculate_server_cost_for_user(
        transaction,
        budget.user as u64,
        begin,
        end,
        None,
    )
    .await?
    else {
        return Err(anyhow!("Unexpected ServerCostForProject variant.").into());
    };
    let over = UserBudgetOverSimple {
        budget_id: budget_id as u32,
        user_id: budget.user,
        user_name: budget.username,
        over: cost.total >= budget.amount as f64,
    };
    overs.push(over);
    Ok(overs)
}

pub async fn calculate_user_budget_over_for_budget_combined(
    transaction: &mut Transaction<'_, MySql>,
    budget_id: u64,
    end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverCombined>, UnexpectedOnlyError> {
    let mut overs = vec![];
    let Some(budget) =
        select_maybe_user_budget_from_db(transaction, budget_id).await?
    else {
        return Ok(overs);
    };
    let year = budget.year;
    let user = select_user_from_db(transaction, budget.user as u64)
        .await
        .context("Failed to select user")?;
    let project_budget =
        select_maybe_project_budget_by_project_and_year_from_db(
            transaction,
            user.project as u64,
            year,
        )
        .await?;
    if year != end.year() as u32 {
        return Ok(overs);
    }
    let begin = start_of_the_year(year);
    let ServerCostForUser::Normal(cost) = calculate_server_cost_for_user(
        transaction,
        budget.user as u64,
        begin,
        end,
        None,
    )
    .await?
    else {
        return Err(anyhow!("Unexpected ServerCostForProject variant.").into());
    };
    let ServerCostForProject::Normal(project_cost) =
        calculate_server_cost_for_project(
            transaction,
            user.project as u64,
            begin,
            end,
            None,
        )
        .await?
    else {
        return Err(anyhow!("Unexpected ServerCostForProject variant.").into());
    };
    let over = UserBudgetOverCombined {
        budget_id: budget_id as u32,
        user_id: budget.user,
        user_name: budget.username,
        project_budget_id: project_budget.clone().map(|b| b.id),
        project_id: user.project,
        project_name: user.project_name,
        over: cost.total >= budget.amount as f64
            || match project_budget {
                Some(project_budget) => {
                    project_cost.total >= project_budget.amount as f64
                }
                None => false,
            },
    };
    overs.push(over);
    Ok(overs)
}

pub async fn calculate_user_budget_over_for_budget_detail(
    transaction: &mut Transaction<'_, MySql>,
    budget_id: u64,
    end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverDetail>, UnexpectedOnlyError> {
    let mut overs = vec![];
    let Some(budget) =
        select_maybe_user_budget_from_db(transaction, budget_id).await?
    else {
        return Ok(overs);
    };
    let year = budget.year;
    if year != end.year() as u32 {
        return Ok(overs);
    }
    let begin = start_of_the_year(year);
    let ServerCostForUser::Normal(cost) = calculate_server_cost_for_user(
        transaction,
        budget.user as u64,
        begin,
        end,
        None,
    )
    .await?
    else {
        return Err(anyhow!("Unexpected ServerCostForProject variant.").into());
    };
    let over = UserBudgetOverDetail {
        budget_id: budget_id as u32,
        user_id: budget.user,
        user_name: budget.username,
        over: cost.total >= budget.amount as f64,
        cost: cost.total,
        budget: budget.amount,
    };
    overs.push(over);
    Ok(overs)
}

pub async fn calculate_user_budget_over_for_budget_combined_detail(
    transaction: &mut Transaction<'_, MySql>,
    budget_id: u64,
    end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverCombinedDetail>, UnexpectedOnlyError> {
    let mut overs = vec![];
    let Some(budget) =
        select_maybe_user_budget_from_db(transaction, budget_id).await?
    else {
        return Ok(overs);
    };
    let year = budget.year;
    let user = select_user_from_db(transaction, budget.user as u64)
        .await
        .context("Failed to select user")?;
    let project_budget =
        select_maybe_project_budget_by_project_and_year_from_db(
            transaction,
            user.project as u64,
            year,
        )
        .await?;
    if year != end.year() as u32 {
        return Ok(overs);
    }
    let begin = start_of_the_year(year);
    let ServerCostForUser::Normal(cost) = calculate_server_cost_for_user(
        transaction,
        budget.user as u64,
        begin,
        end,
        None,
    )
    .await?
    else {
        return Err(anyhow!("Unexpected ServerCostForProject variant.").into());
    };
    let ServerCostForProject::Normal(project_cost) =
        calculate_server_cost_for_project(
            transaction,
            user.project as u64,
            begin,
            end,
            None,
        )
        .await?
    else {
        return Err(anyhow!("Unexpected ServerCostForProject variant.").into());
    };
    let over = UserBudgetOverCombinedDetail {
        budget_id: budget_id as u32,
        user_id: budget.user,
        user_name: budget.username,
        project_budget_id: project_budget.clone().map(|b| b.id),
        project_id: user.project,
        project_name: user.project_name,
        over: cost.total >= budget.amount as f64
            || match project_budget.clone() {
                Some(project_budget) => {
                    project_cost.total >= project_budget.amount as f64
                }
                None => false,
            },
        project_cost: project_cost.total,
        project_budget: project_budget.map(|b| b.amount),
        user_cost: cost.total,
        user_budget: budget.amount,
    };
    overs.push(over);
    Ok(overs)
}

pub async fn calculate_user_budget_over_for_budget(
    transaction: &mut Transaction<'_, MySql>,
    budget_id: u64,
    end: DateTime<Utc>,
    combined: Option<bool>,
    detail: Option<bool>,
) -> Result<UserBudgetOver, UnexpectedOnlyError> {
    Ok(match (combined, detail) {
        (Some(true), Some(true)) => UserBudgetOver::CombinedDetail(
            calculate_user_budget_over_for_budget_combined_detail(
                transaction,
                budget_id,
                end,
            )
            .await?,
        ),
        (None | Some(false), Some(true)) => UserBudgetOver::Detail(
            calculate_user_budget_over_for_budget_detail(
                transaction,
                budget_id,
                end,
            )
            .await?,
        ),
        (Some(true), None | Some(false)) => UserBudgetOver::Combined(
            calculate_user_budget_over_for_budget_combined(
                transaction,
                budget_id,
                end,
            )
            .await?,
        ),
        (None | Some(false), None | Some(false)) => UserBudgetOver::Normal(
            calculate_user_budget_over_for_budget_normal(
                transaction,
                budget_id,
                end,
            )
            .await?,
        ),
    })
}

pub async fn calculate_user_budget_over_for_user_normal(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverSimple>, UnexpectedOnlyError> {
    let mut overs = vec![];
    let year = end.year() as u32;
    let Some(budget) = select_maybe_user_budget_by_user_and_year_from_db(
        transaction,
        user_id,
        year,
    )
    .await?
    else {
        return Ok(overs);
    };
    let begin = start_of_the_year(year);
    let ServerCostForUser::Normal(cost) = calculate_server_cost_for_user(
        transaction,
        budget.user as u64,
        begin,
        end,
        None,
    )
    .await?
    else {
        return Err(anyhow!("Unexpected ServerCostForProject variant.").into());
    };
    let over = UserBudgetOverSimple {
        budget_id: budget.id,
        user_id: budget.user,
        user_name: budget.username,
        over: cost.total >= budget.amount as f64,
    };
    overs.push(over);
    Ok(overs)
}

pub async fn calculate_user_budget_over_for_user_combined(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverCombined>, UnexpectedOnlyError> {
    let mut overs = vec![];
    let year = end.year() as u32;
    let Some(budget) = select_maybe_user_budget_by_user_and_year_from_db(
        transaction,
        user_id,
        year,
    )
    .await?
    else {
        return Ok(overs);
    };
    let user = select_user_from_db(transaction, budget.user as u64)
        .await
        .context("Failed to select user")?;
    let project_budget =
        select_maybe_project_budget_by_project_and_year_from_db(
            transaction,
            user.project as u64,
            year,
        )
        .await?;
    if year != end.year() as u32 {
        return Ok(overs);
    }
    let begin = start_of_the_year(year);
    let ServerCostForUser::Normal(cost) = calculate_server_cost_for_user(
        transaction,
        budget.user as u64,
        begin,
        end,
        None,
    )
    .await?
    else {
        return Err(anyhow!("Unexpected ServerCostForProject variant.").into());
    };
    let ServerCostForProject::Normal(project_cost) =
        calculate_server_cost_for_project(
            transaction,
            user.project as u64,
            begin,
            end,
            None,
        )
        .await?
    else {
        return Err(anyhow!("Unexpected ServerCostForProject variant.").into());
    };
    let over = UserBudgetOverCombined {
        budget_id: budget.id,
        user_id: budget.user,
        user_name: budget.username,
        project_budget_id: project_budget.clone().map(|b| b.id),
        project_id: user.project,
        project_name: user.project_name,
        over: cost.total >= budget.amount as f64
            || match project_budget {
                Some(project_budget) => {
                    project_cost.total >= project_budget.amount as f64
                }
                None => false,
            },
    };
    overs.push(over);
    Ok(overs)
}

pub async fn calculate_user_budget_over_for_user_detail(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverDetail>, UnexpectedOnlyError> {
    let mut overs = vec![];
    let year = end.year() as u32;
    let Some(budget) = select_maybe_user_budget_by_user_and_year_from_db(
        transaction,
        user_id,
        year,
    )
    .await?
    else {
        return Ok(overs);
    };
    let begin = start_of_the_year(year);
    let ServerCostForUser::Normal(cost) = calculate_server_cost_for_user(
        transaction,
        budget.user as u64,
        begin,
        end,
        None,
    )
    .await?
    else {
        return Err(anyhow!("Unexpected ServerCostForProject variant.").into());
    };
    let over = UserBudgetOverDetail {
        budget_id: budget.id,
        user_id: budget.user,
        user_name: budget.username,
        over: cost.total >= budget.amount as f64,
        cost: cost.total,
        budget: budget.amount,
    };
    overs.push(over);
    Ok(overs)
}

pub async fn calculate_user_budget_over_for_user_combined_detail(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverCombinedDetail>, UnexpectedOnlyError> {
    let mut overs = vec![];
    let year = end.year() as u32;
    let Some(budget) = select_maybe_user_budget_by_user_and_year_from_db(
        transaction,
        user_id,
        year,
    )
    .await?
    else {
        return Ok(overs);
    };
    let user = select_user_from_db(transaction, budget.user as u64)
        .await
        .context("Failed to select user")?;
    let project_budget =
        select_maybe_project_budget_by_project_and_year_from_db(
            transaction,
            user.project as u64,
            year,
        )
        .await?;
    if year != end.year() as u32 {
        return Ok(overs);
    }
    let begin = start_of_the_year(year);
    let ServerCostForUser::Normal(cost) = calculate_server_cost_for_user(
        transaction,
        budget.user as u64,
        begin,
        end,
        None,
    )
    .await?
    else {
        return Err(anyhow!("Unexpected ServerCostForProject variant.").into());
    };
    let ServerCostForProject::Normal(project_cost) =
        calculate_server_cost_for_project(
            transaction,
            user.project as u64,
            begin,
            end,
            None,
        )
        .await?
    else {
        return Err(anyhow!("Unexpected ServerCostForProject variant.").into());
    };
    let over = UserBudgetOverCombinedDetail {
        budget_id: budget.id,
        user_id: budget.user,
        user_name: budget.username,
        project_budget_id: project_budget.clone().map(|b| b.id),
        project_id: user.project,
        project_name: user.project_name,
        over: cost.total >= budget.amount as f64
            || match project_budget.clone() {
                Some(project_budget) => {
                    project_cost.total >= project_budget.amount as f64
                }
                None => false,
            },
        project_cost: project_cost.total,
        project_budget: project_budget.map(|b| b.amount),
        user_cost: cost.total,
        user_budget: budget.amount,
    };
    overs.push(over);
    Ok(overs)
}

pub async fn calculate_user_budget_over_for_user(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    end: DateTime<Utc>,
    combined: Option<bool>,
    detail: Option<bool>,
) -> Result<UserBudgetOver, UnexpectedOnlyError> {
    Ok(match (combined, detail) {
        (Some(true), Some(true)) => UserBudgetOver::CombinedDetail(
            calculate_user_budget_over_for_user_combined_detail(
                transaction,
                user_id,
                end,
            )
            .await?,
        ),
        (None | Some(false), Some(true)) => UserBudgetOver::Detail(
            calculate_user_budget_over_for_user_detail(
                transaction,
                user_id,
                end,
            )
            .await?,
        ),
        (Some(true), None | Some(false)) => UserBudgetOver::Combined(
            calculate_user_budget_over_for_user_combined(
                transaction,
                user_id,
                end,
            )
            .await?,
        ),
        (None | Some(false), None | Some(false)) => UserBudgetOver::Normal(
            calculate_user_budget_over_for_user_normal(
                transaction,
                user_id,
                end,
            )
            .await?,
        ),
    })
}

pub async fn calculate_user_budget_over_for_project_normal(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
    end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverSimple>, UnexpectedOnlyError> {
    let mut overs = vec![];
    let year = end.year() as u32;
    let budgets = select_user_budgets_by_project_and_year_from_db(
        transaction,
        project_id,
        year,
    )
    .await?;
    let begin = start_of_the_year(year);
    for budget in budgets {
        let ServerCostForUser::Normal(cost) = calculate_server_cost_for_user(
            transaction,
            budget.user as u64,
            begin,
            end,
            None,
        )
        .await?
        else {
            return Err(
                anyhow!("Unexpected ServerCostForProject variant.").into()
            );
        };
        let over = UserBudgetOverSimple {
            budget_id: budget.id,
            user_id: budget.user,
            user_name: budget.username,
            over: cost.total >= budget.amount as f64,
        };
        overs.push(over);
    }
    Ok(overs)
}

pub async fn calculate_user_budget_over_for_project_combined(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
    end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverCombined>, UnexpectedOnlyError> {
    let mut overs = vec![];
    let year = end.year() as u32;
    if year != end.year() as u32 {
        return Ok(overs);
    }
    let budgets = select_user_budgets_by_project_and_year_from_db(
        transaction,
        project_id,
        year,
    )
    .await?;
    for budget in budgets {
        // TODO: doing all the calculations in a loop is inefficient
        let user = select_user_from_db(transaction, budget.user as u64)
            .await
            .context("Failed to select user")?;
        let project_budget =
            select_maybe_project_budget_by_project_and_year_from_db(
                transaction,
                user.project as u64,
                year,
            )
            .await?;
        let begin = start_of_the_year(year);
        let ServerCostForUser::Normal(cost) = calculate_server_cost_for_user(
            transaction,
            budget.user as u64,
            begin,
            end,
            None,
        )
        .await?
        else {
            return Err(
                anyhow!("Unexpected ServerCostForProject variant.").into()
            );
        };
        let ServerCostForProject::Normal(project_cost) =
            calculate_server_cost_for_project(
                transaction,
                user.project as u64,
                begin,
                end,
                None,
            )
            .await?
        else {
            return Err(
                anyhow!("Unexpected ServerCostForProject variant.").into()
            );
        };
        let over = UserBudgetOverCombined {
            budget_id: budget.id,
            user_id: budget.user,
            user_name: budget.username,
            project_budget_id: project_budget.clone().map(|b| b.id),
            project_id: user.project,
            project_name: user.project_name,
            over: cost.total >= budget.amount as f64
                || match project_budget.clone() {
                    Some(project_budget) => {
                        project_cost.total >= project_budget.amount as f64
                    }
                    None => false,
                },
        };
        overs.push(over);
    }
    Ok(overs)
}

pub async fn calculate_user_budget_over_for_project_detail(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
    end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverDetail>, UnexpectedOnlyError> {
    let mut overs = vec![];
    let year = end.year() as u32;
    let budgets = select_user_budgets_by_project_and_year_from_db(
        transaction,
        project_id,
        year,
    )
    .await?;
    let begin = start_of_the_year(year);
    for budget in budgets {
        let ServerCostForUser::Normal(cost) = calculate_server_cost_for_user(
            transaction,
            budget.user as u64,
            begin,
            end,
            None,
        )
        .await?
        else {
            return Err(
                anyhow!("Unexpected ServerCostForProject variant.").into()
            );
        };
        let over = UserBudgetOverDetail {
            budget_id: budget.id,
            user_id: budget.user,
            user_name: budget.username,
            over: cost.total >= budget.amount as f64,
            cost: cost.total,
            budget: budget.amount,
        };
        overs.push(over);
    }
    Ok(overs)
}

pub async fn calculate_user_budget_over_for_project_combined_detail(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
    end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverCombinedDetail>, UnexpectedOnlyError> {
    let mut overs = vec![];
    let year = end.year() as u32;
    if year != end.year() as u32 {
        return Ok(overs);
    }
    let budgets = select_user_budgets_by_project_and_year_from_db(
        transaction,
        project_id,
        year,
    )
    .await?;
    for budget in budgets {
        // TODO: doing all the calculations in a loop is inefficient
        let user = select_user_from_db(transaction, budget.user as u64)
            .await
            .context("Failed to select user")?;
        let project_budget =
            select_maybe_project_budget_by_project_and_year_from_db(
                transaction,
                user.project as u64,
                year,
            )
            .await?;
        let begin = start_of_the_year(year);
        let ServerCostForUser::Normal(cost) = calculate_server_cost_for_user(
            transaction,
            budget.user as u64,
            begin,
            end,
            None,
        )
        .await?
        else {
            return Err(
                anyhow!("Unexpected ServerCostForProject variant.").into()
            );
        };
        let ServerCostForProject::Normal(project_cost) =
            calculate_server_cost_for_project(
                transaction,
                user.project as u64,
                begin,
                end,
                None,
            )
            .await?
        else {
            return Err(
                anyhow!("Unexpected ServerCostForProject variant.").into()
            );
        };
        let over = UserBudgetOverCombinedDetail {
            budget_id: budget.id,
            user_id: budget.user,
            user_name: budget.username,
            project_budget_id: project_budget.clone().map(|b| b.id),
            project_id: user.project,
            project_name: user.project_name,
            over: cost.total >= budget.amount as f64
                || match project_budget.clone() {
                    Some(project_budget) => {
                        project_cost.total >= project_budget.amount as f64
                    }
                    None => false,
                },
            project_cost: project_cost.total,
            project_budget: project_budget.map(|b| b.amount),
            user_cost: cost.total,
            user_budget: budget.amount,
        };
        overs.push(over);
    }
    Ok(overs)
}

pub async fn calculate_user_budget_over_for_project(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
    end: DateTime<Utc>,
    combined: Option<bool>,
    detail: Option<bool>,
) -> Result<UserBudgetOver, UnexpectedOnlyError> {
    Ok(match (combined, detail) {
        (Some(true), Some(true)) => UserBudgetOver::CombinedDetail(
            calculate_user_budget_over_for_project_combined_detail(
                transaction,
                project_id,
                end,
            )
            .await?,
        ),
        (None | Some(false), Some(true)) => UserBudgetOver::Detail(
            calculate_user_budget_over_for_project_detail(
                transaction,
                project_id,
                end,
            )
            .await?,
        ),
        (Some(true), None | Some(false)) => UserBudgetOver::Combined(
            calculate_user_budget_over_for_project_combined(
                transaction,
                project_id,
                end,
            )
            .await?,
        ),
        (None | Some(false), None | Some(false)) => UserBudgetOver::Normal(
            calculate_user_budget_over_for_project_normal(
                transaction,
                project_id,
                end,
            )
            .await?,
        ),
    })
}

pub async fn calculate_user_budget_over_for_all_normal(
    transaction: &mut Transaction<'_, MySql>,
    end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverSimple>, UnexpectedOnlyError> {
    let mut overs = vec![];
    let year = end.year() as u32;
    let budgets =
        select_user_budgets_by_year_from_db(transaction, year).await?;
    let begin = start_of_the_year(year);
    for budget in budgets {
        let ServerCostForUser::Normal(cost) = calculate_server_cost_for_user(
            transaction,
            budget.user as u64,
            begin,
            end,
            None,
        )
        .await?
        else {
            return Err(
                anyhow!("Unexpected ServerCostForProject variant.").into()
            );
        };
        let over = UserBudgetOverSimple {
            budget_id: budget.id,
            user_id: budget.user,
            user_name: budget.username,
            over: cost.total >= budget.amount as f64,
        };
        overs.push(over);
    }
    Ok(overs)
}

pub async fn calculate_user_budget_over_for_all_combined(
    transaction: &mut Transaction<'_, MySql>,
    end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverCombined>, UnexpectedOnlyError> {
    let mut overs = vec![];
    let year = end.year() as u32;
    if year != end.year() as u32 {
        return Ok(overs);
    }
    let budgets =
        select_user_budgets_by_year_from_db(transaction, year).await?;
    for budget in budgets {
        // TODO: doing all the calculations in a loop is inefficient
        let user = select_user_from_db(transaction, budget.user as u64)
            .await
            .context("Failed to select user")?;
        let project_budget =
            select_maybe_project_budget_by_project_and_year_from_db(
                transaction,
                user.project as u64,
                year,
            )
            .await?;
        let begin = start_of_the_year(year);
        let ServerCostForUser::Normal(cost) = calculate_server_cost_for_user(
            transaction,
            budget.user as u64,
            begin,
            end,
            None,
        )
        .await?
        else {
            return Err(
                anyhow!("Unexpected ServerCostForProject variant.").into()
            );
        };
        let ServerCostForProject::Normal(project_cost) =
            calculate_server_cost_for_project(
                transaction,
                user.project as u64,
                begin,
                end,
                None,
            )
            .await?
        else {
            return Err(
                anyhow!("Unexpected ServerCostForProject variant.").into()
            );
        };
        let over = UserBudgetOverCombined {
            budget_id: budget.id,
            user_id: budget.user,
            user_name: budget.username,
            project_budget_id: project_budget.clone().map(|b| b.id),
            project_id: user.project,
            project_name: user.project_name,
            over: cost.total >= budget.amount as f64
                || match project_budget.clone() {
                    Some(project_budget) => {
                        project_cost.total >= project_budget.amount as f64
                    }
                    None => false,
                },
        };
        overs.push(over);
    }
    Ok(overs)
}

pub async fn calculate_user_budget_over_for_all_detail(
    transaction: &mut Transaction<'_, MySql>,
    end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverDetail>, UnexpectedOnlyError> {
    let mut overs = vec![];
    let year = end.year() as u32;
    let budgets =
        select_user_budgets_by_year_from_db(transaction, year).await?;
    let begin = start_of_the_year(year);
    for budget in budgets {
        let ServerCostForUser::Normal(cost) = calculate_server_cost_for_user(
            transaction,
            budget.user as u64,
            begin,
            end,
            None,
        )
        .await?
        else {
            return Err(
                anyhow!("Unexpected ServerCostForProject variant.").into()
            );
        };
        let over = UserBudgetOverDetail {
            budget_id: budget.id,
            user_id: budget.user,
            user_name: budget.username,
            over: cost.total >= budget.amount as f64,
            cost: cost.total,
            budget: budget.amount,
        };
        overs.push(over);
    }
    Ok(overs)
}

pub async fn calculate_user_budget_over_for_all_combined_detail(
    transaction: &mut Transaction<'_, MySql>,
    end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverCombinedDetail>, UnexpectedOnlyError> {
    let mut overs = vec![];
    let year = end.year() as u32;
    if year != end.year() as u32 {
        return Ok(overs);
    }
    let budgets =
        select_user_budgets_by_year_from_db(transaction, year).await?;
    for budget in budgets {
        // TODO: doing all the calculations in a loop is inefficient
        let user = select_user_from_db(transaction, budget.user as u64)
            .await
            .context("Failed to select user")?;
        let project_budget =
            select_maybe_project_budget_by_project_and_year_from_db(
                transaction,
                user.project as u64,
                year,
            )
            .await?;
        let begin = start_of_the_year(year);
        let ServerCostForUser::Normal(cost) = calculate_server_cost_for_user(
            transaction,
            budget.user as u64,
            begin,
            end,
            None,
        )
        .await?
        else {
            return Err(
                anyhow!("Unexpected ServerCostForProject variant.").into()
            );
        };
        let ServerCostForProject::Normal(project_cost) =
            calculate_server_cost_for_project(
                transaction,
                user.project as u64,
                begin,
                end,
                None,
            )
            .await?
        else {
            return Err(
                anyhow!("Unexpected ServerCostForProject variant.").into()
            );
        };
        let over = UserBudgetOverCombinedDetail {
            budget_id: budget.id,
            user_id: budget.user,
            user_name: budget.username,
            project_budget_id: project_budget.clone().map(|b| b.id),
            project_id: user.project,
            project_name: user.project_name,
            over: cost.total >= budget.amount as f64
                || match project_budget.clone() {
                    Some(project_budget) => {
                        project_cost.total >= project_budget.amount as f64
                    }
                    None => false,
                },
            project_cost: project_cost.total,
            project_budget: project_budget.map(|b| b.amount),
            user_cost: cost.total,
            user_budget: budget.amount,
        };
        overs.push(over);
    }
    Ok(overs)
}

pub async fn calculate_user_budget_over_for_all(
    transaction: &mut Transaction<'_, MySql>,
    end: DateTime<Utc>,
    combined: Option<bool>,
    detail: Option<bool>,
) -> Result<UserBudgetOver, UnexpectedOnlyError> {
    Ok(match (combined, detail) {
        (Some(true), Some(true)) => UserBudgetOver::CombinedDetail(
            calculate_user_budget_over_for_all_combined_detail(
                transaction,
                end,
            )
            .await?,
        ),
        (None | Some(false), Some(true)) => UserBudgetOver::Detail(
            calculate_user_budget_over_for_all_detail(transaction, end).await?,
        ),
        (Some(true), None | Some(false)) => UserBudgetOver::Combined(
            calculate_user_budget_over_for_all_combined(transaction, end)
                .await?,
        ),
        (None | Some(false), None | Some(false)) => UserBudgetOver::Normal(
            calculate_user_budget_over_for_all_normal(transaction, end).await?,
        ),
    })
}

#[tracing::instrument(name = "user_budget_over")]
pub async fn user_budget_over(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Query<UserBudgetOverParams>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    // TODO: add proper permission check
    let end = params.end.unwrap_or(Utc::now().fixed_offset());
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let over = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        calculate_user_budget_over_for_all(
            &mut transaction,
            end.into(),
            params.combined,
            params.detail,
        )
        .await?
    } else if let Some(project_id) = params.project {
        require_master_user_or_return_not_found(&user, project_id)?;
        calculate_user_budget_over_for_project(
            &mut transaction,
            project_id as u64,
            end.into(),
            params.combined,
            params.detail,
        )
        .await?
    } else if let Some(user_id) = params.user {
        let user_queried =
            select_user_from_db(&mut transaction, user_id as u64).await?;
        require_user_or_project_master_or_not_found(
            &user,
            user_id,
            user_queried.project,
        )?;
        calculate_user_budget_over_for_user(
            &mut transaction,
            user_id as u64,
            end.into(),
            params.combined,
            params.detail,
        )
        .await?
    } else if let Some(budget_id) = params.budget {
        let user_budget =
            select_user_budget_from_db(&mut transaction, budget_id as u64)
                .await?;
        let user_budget_user =
            select_user_from_db(&mut transaction, user_budget.user as u64)
                .await?;
        require_user_or_project_master_or_not_found(
            &user,
            user_budget_user.id,
            user_budget_user.project,
        )?;
        calculate_user_budget_over_for_budget(
            &mut transaction,
            budget_id as u64,
            end.into(),
            params.combined,
            params.detail,
        )
        .await?
    } else {
        calculate_user_budget_over_for_user(
            &mut transaction,
            user.id as u64,
            end.into(),
            params.combined,
            params.detail,
        )
        .await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(over))
}
