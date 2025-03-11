use crate::authorization::require_admin_user;
use crate::error::{OptionApiError, UnexpectedOnlyError};
use actix_web::web::{Data, Query, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use chrono::{DateTime, Utc};
use lrzcc_wire::budgeting::{
    UserBudgetOverCombined, UserBudgetOverCombinedDetail, UserBudgetOverDetail,
    UserBudgetOverParams, UserBudgetOverSimple,
};
use lrzcc_wire::user::{Project, User};
use serde::Serialize;
use sqlx::{MySql, MySqlPool, Transaction};

#[derive(Serialize)]
#[serde(untagged)]
// TODO: remove this
#[allow(dead_code)]
pub enum UserBudgetOver {
    Normal(Vec<UserBudgetOverSimple>),
    Combined(Vec<UserBudgetOverCombined>),
    Detail(Vec<UserBudgetOverDetail>),
    CombinedDetail(Vec<UserBudgetOverCombinedDetail>),
}

pub async fn calculate_user_budget_over_for_budget_normal(
    _transaction: &mut Transaction<'_, MySql>,
    _budget_id: u64,
    _end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverSimple>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_user_budget_over_for_budget_combined(
    _transaction: &mut Transaction<'_, MySql>,
    _budget_id: u64,
    _end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverCombined>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_user_budget_over_for_budget_detail(
    _transaction: &mut Transaction<'_, MySql>,
    _budget_id: u64,
    _end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverDetail>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_user_budget_over_for_budget_combined_detail(
    _transaction: &mut Transaction<'_, MySql>,
    _budget_id: u64,
    _end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverCombinedDetail>, UnexpectedOnlyError> {
    todo!()
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
    _transaction: &mut Transaction<'_, MySql>,
    _user_id: u64,
    _end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverSimple>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_user_budget_over_for_user_combined(
    _transaction: &mut Transaction<'_, MySql>,
    _user_id: u64,
    _end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverCombined>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_user_budget_over_for_user_detail(
    _transaction: &mut Transaction<'_, MySql>,
    _user_id: u64,
    _end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverDetail>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_user_budget_over_for_user_combined_detail(
    _transaction: &mut Transaction<'_, MySql>,
    _user_id: u64,
    _end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverCombinedDetail>, UnexpectedOnlyError> {
    todo!()
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
    _transaction: &mut Transaction<'_, MySql>,
    _project_id: u64,
    _end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverSimple>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_user_budget_over_for_project_combined(
    _transaction: &mut Transaction<'_, MySql>,
    _project_id: u64,
    _end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverCombined>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_user_budget_over_for_project_detail(
    _transaction: &mut Transaction<'_, MySql>,
    _project_id: u64,
    _end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverDetail>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_user_budget_over_for_project_combined_detail(
    _transaction: &mut Transaction<'_, MySql>,
    _project_id: u64,
    _end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverCombinedDetail>, UnexpectedOnlyError> {
    todo!()
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
    _transaction: &mut Transaction<'_, MySql>,
    _end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverSimple>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_user_budget_over_for_all_combined(
    _transaction: &mut Transaction<'_, MySql>,
    _end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverCombined>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_user_budget_over_for_all_detail(
    _transaction: &mut Transaction<'_, MySql>,
    _end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverDetail>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_user_budget_over_for_all_combined_detail(
    _transaction: &mut Transaction<'_, MySql>,
    _end: DateTime<Utc>,
) -> Result<Vec<UserBudgetOverCombinedDetail>, UnexpectedOnlyError> {
    todo!()
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
    // TODO: not necessary?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<UserBudgetOverParams>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    // TODO: add proper permission check
    require_admin_user(&user)?;
    let end = params.end.unwrap_or(Utc::now().fixed_offset());
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let over = if params.all.unwrap_or(false) {
        calculate_user_budget_over_for_all(
            &mut transaction,
            end.into(),
            params.combined,
            params.detail,
        )
        .await?
    } else if let Some(project_id) = params.project {
        calculate_user_budget_over_for_project(
            &mut transaction,
            project_id as u64,
            end.into(),
            params.combined,
            params.detail,
        )
        .await?
    } else if let Some(user_id) = params.user {
        calculate_user_budget_over_for_user(
            &mut transaction,
            user_id as u64,
            end.into(),
            params.combined,
            params.detail,
        )
        .await?
    } else if let Some(budget_id) = params.budget {
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
