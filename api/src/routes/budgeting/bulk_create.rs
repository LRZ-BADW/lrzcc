use actix_web::{
    HttpResponse,
    web::{Data, Json, ReqData},
};
use anyhow::Context;
use avina_wire::{
    budgeting::{BudgetBulkCreate, BudgetBulkCreateData},
    user::User,
};
use sqlx::{MySql, MySqlPool, Transaction};

use crate::{
    authorization::require_admin_user,
    database::{
        budgeting::{
            project_budget::{
                NewProjectBudget, insert_project_budget_into_db,
                select_project_budgets_by_year_from_db,
            },
            user_budget::{
                NewUserBudget, insert_user_budget_into_db,
                select_user_budgets_by_year_from_db,
            },
        },
        user::{
            project::select_all_projects_from_db,
            user::select_all_users_from_db,
        },
    },
    error::{MinimalApiError, NormalApiError},
};

async fn bulk_create_user_budgets(
    transaction: &mut Transaction<'_, MySql>,
    year: u32,
) -> Result<u32, MinimalApiError> {
    let users = select_all_users_from_db(transaction).await?;
    let budget_user_ids =
        select_user_budgets_by_year_from_db(transaction, year)
            .await?
            .iter()
            .map(|b| b.user)
            .collect::<Vec<_>>();
    // TODO: this is inefficient, do a bulk insert.
    let mut count = 0;
    for user in users.iter().filter(|u| !budget_user_ids.contains(&u.id)) {
        insert_user_budget_into_db(
            transaction,
            &NewUserBudget {
                user_id: user.id as u64,
                year,
                amount: 0,
            },
        )
        .await?;
        count += 1;
    }
    Ok(count)
}

async fn bulk_create_project_budgets(
    transaction: &mut Transaction<'_, MySql>,
    year: u32,
) -> Result<u32, MinimalApiError> {
    let projects = select_all_projects_from_db(transaction).await?;
    let budget_project_ids =
        select_project_budgets_by_year_from_db(transaction, year)
            .await?
            .iter()
            .map(|b| b.project)
            .collect::<Vec<_>>();
    // TODO: this is inefficient, do a bulk insert.
    let mut count = 0;
    for project in projects
        .iter()
        .filter(|p| !budget_project_ids.contains(&p.id))
    {
        insert_project_budget_into_db(
            transaction,
            &NewProjectBudget {
                project_id: project.id as u64,
                year,
                amount: 0,
            },
        )
        .await?;
        count += 1;
    }
    Ok(count)
}

#[tracing::instrument(name = "budget_bulk_create")]
pub async fn budget_bulk_create(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<BudgetBulkCreateData>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let new_user_budget_count =
        bulk_create_user_budgets(&mut transaction, data.year as u32).await?;
    let new_project_budget_count =
        bulk_create_project_budgets(&mut transaction, data.year as u32).await?;
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
