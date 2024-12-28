use crate::authorization::require_admin_user;
use crate::database::budgeting::user_budget::sync_user_budgets_in_db;
use crate::error::NormalApiError;
use actix_web::web::{Data, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::budgeting::UserBudgetSync;
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

// TODO: write tests for this endpoint
#[tracing::instrument(name = "user_budget_sync")]
pub async fn user_budget_sync(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    // TODO: this can only be an auth or unexpeceted error, we need a type for that
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let count = sync_user_budgets_in_db(&mut transaction).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok().content_type("application/json").json(
        UserBudgetSync {
            updated_budget_count: count as u32,
        },
    ))
}
