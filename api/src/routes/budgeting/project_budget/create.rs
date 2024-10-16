use crate::authorization::require_admin_user;
use crate::database::user::project::select_project_name_from_db;
use crate::error::{MinimalApiError, NormalApiError, OptionApiError};
use actix_web::web::{Data, Json, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use chrono::{Datelike, Utc};
use lrzcc_wire::budgeting::{ProjectBudget, ProjectBudgetCreateData};
use lrzcc_wire::user::{Project, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

pub struct NewProjectBudget {
    pub project_id: u64,
    pub year: u32,
    pub amount: i64,
}

impl TryFrom<ProjectBudgetCreateData> for NewProjectBudget {
    type Error = String;

    fn try_from(data: ProjectBudgetCreateData) -> Result<Self, Self::Error> {
        Ok(Self {
            project_id: data.project as u64,
            year: data.year.unwrap_or(Utc::now().year() as u32),
            amount: data.amount.unwrap_or(0),
        })
    }
}

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

#[tracing::instrument(
    name = "insert_project_budget_into_db",
    skip(new_project_budget, transaction)
)]
pub async fn insert_project_budget_into_db(
    transaction: &mut Transaction<'_, MySql>,
    new_project_budget: &NewProjectBudget,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query = sqlx::query!(
        r#"
        INSERT IGNORE INTO budgeting_projectbudget (year, amount, project_id)
        VALUES (?, ?, ?)
        "#,
        new_project_budget.year,
        new_project_budget.amount,
        new_project_budget.project_id,
    );
    let result = transaction
        .execute(query)
        .await
        .context("Failed to execute insert query")?;
    if result.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new quota, a conflicting entry exists"
                .to_string(),
        ));
    }
    let id = result.last_insert_id();
    Ok(id)
}
