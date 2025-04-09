use actix_web::{
    web::{Data, Path, ReqData},
    HttpResponse,
};
use anyhow::Context;
use lrzcc_wire::{
    budgeting::{BudgetBulkCreate, BudgetBulkCreateData},
    user::{Project, User},
};
use sqlx::{MySql, MySqlPool, Transaction};

use crate::{
    authorization::require_admin_user,
    error::{NormalApiError, UnexpectedOnlyError},
};

async fn bulk_create_user_budgets(
    _transaction: &mut Transaction<'_, MySql>,
    _year: u32,
) -> Result<u32, UnexpectedOnlyError> {
    todo!()
}

async fn bulk_create_project_budgets(
    _transaction: &mut Transaction<'_, MySql>,
    _year: u32,
) -> Result<u32, UnexpectedOnlyError> {
    todo!()
}

#[tracing::instrument(name = "budget_bulk_create")]
pub async fn budget_bulk_create(
    user: ReqData<User>,
    // TODO: not necessary?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<BudgetBulkCreateData>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let new_user_budget_count =
        bulk_create_user_budgets(&mut transaction, params.year as u32).await?;
    let new_project_budget_count =
        bulk_create_project_budgets(&mut transaction, params.year as u32)
            .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok().content_type("application/json").json(
        BudgetBulkCreate {
            new_user_budget_count,
            new_project_budget_count,
        },
    ))
}
