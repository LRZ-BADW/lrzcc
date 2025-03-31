use actix_web::{
    web::{Data, Path, ReqData},
    HttpResponse,
};
use anyhow::Context;
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

use super::ProjectBudgetIdParam;
use crate::{
    authorization::require_admin_user,
    database::budgeting::project_budget::select_project_budget_from_db,
    error::OptionApiError,
};

#[tracing::instrument(name = "project_budget_get")]
pub async fn project_budget_get(
    user: ReqData<User>,
    // TODO: not necessary?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<ProjectBudgetIdParam>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let project_budget = select_project_budget_from_db(
        &mut transaction,
        params.project_budget_id as u64,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(project_budget))
}
