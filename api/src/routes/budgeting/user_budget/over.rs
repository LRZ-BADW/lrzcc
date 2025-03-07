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

pub async fn calculate_user_budget_over_for_budget(
    _transaction: &mut Transaction<'_, MySql>,
    _budget_id: u64,
    _end: DateTime<Utc>,
    _combined: Option<bool>,
    _detail: Option<bool>,
) -> Result<UserBudgetOver, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_user_budget_over_for_user(
    _transaction: &mut Transaction<'_, MySql>,
    _user_id: u64,
    _end: DateTime<Utc>,
    _combined: Option<bool>,
    _detail: Option<bool>,
) -> Result<UserBudgetOver, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_user_budget_over_for_project(
    _transaction: &mut Transaction<'_, MySql>,
    _project_id: u64,
    _end: DateTime<Utc>,
    _combined: Option<bool>,
    _detail: Option<bool>,
) -> Result<UserBudgetOver, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_user_budget_over_for_all(
    _transaction: &mut Transaction<'_, MySql>,
    _end: DateTime<Utc>,
    _combined: Option<bool>,
    _detail: Option<bool>,
) -> Result<UserBudgetOver, UnexpectedOnlyError> {
    todo!()
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
