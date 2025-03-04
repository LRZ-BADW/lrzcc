use crate::authorization::require_admin_user;
use crate::error::{OptionApiError, UnexpectedOnlyError};
use actix_web::web::{Data, Query, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use chrono::{DateTime, Utc};
use lrzcc_wire::budgeting::{
    ProjectBudgetOverDetail, ProjectBudgetOverParams, ProjectBudgetOverSimple,
};
use lrzcc_wire::user::{Project, User};
use serde::Serialize;
use sqlx::{MySql, MySqlPool, Transaction};

#[derive(Serialize)]
#[serde(untagged)]
pub enum ProjectBudgetOver {
    Normal(Vec<ProjectBudgetOverSimple>),
    Detail(Vec<ProjectBudgetOverDetail>),
}
pub async fn calculate_project_budget_over_for_budget_normal(
    _transaction: &mut Transaction<'_, MySql>,
    _budget_id: u64,
    _end: DateTime<Utc>,
) -> Result<Vec<ProjectBudgetOverSimple>, UnexpectedOnlyError> {
    todo!();
}

pub async fn calculate_project_budget_over_for_budget_detail(
    _transaction: &mut Transaction<'_, MySql>,
    _budget_id: u64,
    _end: DateTime<Utc>,
) -> Result<Vec<ProjectBudgetOverDetail>, UnexpectedOnlyError> {
    todo!();
}

pub async fn calculate_project_budget_over_for_budget(
    transaction: &mut Transaction<'_, MySql>,
    budget_id: u64,
    end: DateTime<Utc>,
    detail: Option<bool>,
) -> Result<ProjectBudgetOver, UnexpectedOnlyError> {
    Ok(match detail {
        Some(true) => ProjectBudgetOver::Detail(
            calculate_project_budget_over_for_budget_detail(
                transaction,
                budget_id,
                end,
            )
            .await?,
        ),
        _ => ProjectBudgetOver::Normal(
            calculate_project_budget_over_for_budget_normal(
                transaction,
                budget_id,
                end,
            )
            .await?,
        ),
    })
}

pub async fn calculate_project_budget_over_for_project_normal(
    _transaction: &mut Transaction<'_, MySql>,
    _project_id: u64,
    _end: DateTime<Utc>,
) -> Result<Vec<ProjectBudgetOverSimple>, UnexpectedOnlyError> {
    todo!();
}

pub async fn calculate_project_budget_over_for_project_detail(
    _transaction: &mut Transaction<'_, MySql>,
    _project_id: u64,
    _end: DateTime<Utc>,
) -> Result<Vec<ProjectBudgetOverDetail>, UnexpectedOnlyError> {
    todo!();
}

pub async fn calculate_project_budget_over_for_project(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
    end: DateTime<Utc>,
    detail: Option<bool>,
) -> Result<ProjectBudgetOver, UnexpectedOnlyError> {
    Ok(match detail {
        Some(true) => ProjectBudgetOver::Detail(
            calculate_project_budget_over_for_project_detail(
                transaction,
                project_id,
                end,
            )
            .await?,
        ),
        _ => ProjectBudgetOver::Normal(
            calculate_project_budget_over_for_project_normal(
                transaction,
                project_id,
                end,
            )
            .await?,
        ),
    })
}

pub async fn calculate_project_budget_over_for_all_normal(
    _transaction: &mut Transaction<'_, MySql>,
    _end: DateTime<Utc>,
) -> Result<Vec<ProjectBudgetOverSimple>, UnexpectedOnlyError> {
    todo!();
}

pub async fn calculate_project_budget_over_for_all_detail(
    _transaction: &mut Transaction<'_, MySql>,
    _end: DateTime<Utc>,
) -> Result<Vec<ProjectBudgetOverDetail>, UnexpectedOnlyError> {
    todo!();
}

pub async fn calculate_project_budget_over_for_all(
    transaction: &mut Transaction<'_, MySql>,
    end: DateTime<Utc>,
    detail: Option<bool>,
) -> Result<ProjectBudgetOver, UnexpectedOnlyError> {
    Ok(match detail {
        Some(true) => ProjectBudgetOver::Detail(
            calculate_project_budget_over_for_all_detail(transaction, end)
                .await?,
        ),
        _ => ProjectBudgetOver::Normal(
            calculate_project_budget_over_for_all_normal(transaction, end)
                .await?,
        ),
    })
}

#[tracing::instrument(name = "project_budget_over")]
pub async fn project_budget_over(
    user: ReqData<User>,
    // TODO: not necessary?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<ProjectBudgetOverParams>,
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
        calculate_project_budget_over_for_all(
            &mut transaction,
            end.into(),
            params.detail,
        )
        .await?
    } else if let Some(project_id) = params.project {
        calculate_project_budget_over_for_project(
            &mut transaction,
            project_id as u64,
            end.into(),
            params.detail,
        )
        .await?
    } else if let Some(budget_id) = params.budget {
        calculate_project_budget_over_for_budget(
            &mut transaction,
            budget_id as u64,
            end.into(),
            params.detail,
        )
        .await?
    } else {
        calculate_project_budget_over_for_project(
            &mut transaction,
            user.project as u64,
            end.into(),
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
