use crate::authorization::{require_admin_user, require_master_user};
use crate::database::budgeting::user_budget::{
    select_all_user_budgets_from_db, select_user_budgets_by_project_from_db,
    select_user_budgets_by_user_from_db, select_user_budgets_by_year_from_db,
};
use crate::database::user::user::select_user_from_db;
use crate::error::NormalApiError;
use actix_web::web::{Data, Query, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::budgeting::UserBudgetListParams;
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

#[tracing::instrument(name = "user_budget_list")]
pub async fn user_budget_list(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<UserBudgetListParams>,
) -> Result<HttpResponse, NormalApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let user_budgets = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        select_all_user_budgets_from_db(&mut transaction).await?
    } else if let Some(project_id) = params.project {
        require_master_user(&user, project_id)?;
        select_user_budgets_by_project_from_db(
            &mut transaction,
            project_id as u64,
        )
        .await?
    } else if let Some(user_id) = params.user {
        // TODO: this can be optimized when the user is the current user
        let user = select_user_from_db(&mut transaction, user_id as u64)
            .await
            .context("Failed to select user")?;
        require_master_user(&user, user.project)?;
        select_user_budgets_by_user_from_db(&mut transaction, user.id as u64)
            .await?
    } else if let Some(year) = params.year {
        require_admin_user(&user)?;
        select_user_budgets_by_year_from_db(&mut transaction, year).await?
    } else {
        select_user_budgets_by_user_from_db(&mut transaction, user.id as u64)
            .await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(user_budgets))
}
