use actix_web::{
    web::{Data, Path, ReqData},
    HttpResponse,
};
use anyhow::Context;
use avina_wire::user::User;
use sqlx::MySqlPool;

use super::UserBudgetIdParam;
use crate::{
    authorization::require_user_or_project_master_or_not_found,
    database::{
        budgeting::user_budget::select_user_budget_from_db,
        user::user::select_user_from_db,
    },
    error::OptionApiError,
};

#[tracing::instrument(name = "user_budget_get")]
pub async fn user_budget_get(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Path<UserBudgetIdParam>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let user_budget = select_user_budget_from_db(
        &mut transaction,
        params.user_budget_id as u64,
    )
    .await?;
    let user_budget_user =
        select_user_from_db(&mut transaction, user_budget.user as u64).await?;
    require_user_or_project_master_or_not_found(
        &user,
        user_budget_user.id,
        user_budget_user.project,
    )?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(user_budget))
}
