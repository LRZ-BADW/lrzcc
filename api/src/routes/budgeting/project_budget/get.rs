use actix_web::{
    web::{Data, Path, ReqData},
    HttpResponse,
};
use anyhow::Context;
use avina_wire::user::User;
use sqlx::MySqlPool;

use super::ProjectBudgetIdParam;
use crate::{
    authorization::require_project_user_or_return_not_found,
    database::budgeting::project_budget::select_project_budget_from_db,
    error::OptionApiError,
};

#[tracing::instrument(name = "project_budget_get")]
pub async fn project_budget_get(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Path<ProjectBudgetIdParam>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let project_budget = select_project_budget_from_db(
        &mut transaction,
        params.project_budget_id as u64,
    )
    .await?;
    require_project_user_or_return_not_found(&user, project_budget.project)?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(project_budget))
}
