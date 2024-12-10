use crate::authorization::require_admin_user;
use crate::database::budgeting::project_budget::{
    insert_project_budget_into_db, NewProjectBudget,
};
use crate::database::user::project::select_project_name_from_db;
use crate::error::{NormalApiError, OptionApiError};
use actix_web::web::{Data, Json, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::budgeting::{ProjectBudget, ProjectBudgetCreateData};
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

#[tracing::instrument(name = "project_budget_create")]
pub async fn project_budget_create(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    data: Json<ProjectBudgetCreateData>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let new_project_budget: NewProjectBudget = data
        .clone()
        .try_into()
        .map_err(NormalApiError::ValidationError)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let project_name =
        select_project_name_from_db(&mut transaction, data.project as u64)
            .await?;
    let id =
        insert_project_budget_into_db(&mut transaction, &new_project_budget)
            .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let project_budget_created = ProjectBudget {
        id: id as u32,
        project: new_project_budget.project_id as u32,
        project_name,
        year: new_project_budget.year,
        amount: new_project_budget.amount as u32,
    };
    Ok(HttpResponse::Created()
        .content_type("application/json")
        .json(project_budget_created))
}
