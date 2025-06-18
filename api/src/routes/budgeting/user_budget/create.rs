use actix_web::{
    web::{Data, Json, ReqData},
    HttpResponse,
};
use anyhow::Context;
use avina_wire::{
    budgeting::{UserBudget, UserBudgetCreateData},
    user::User,
};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::{
        budgeting::user_budget::{insert_user_budget_into_db, NewUserBudget},
        user::user::select_user_name_from_db,
    },
    error::{NormalApiError, OptionApiError},
};

#[tracing::instrument(name = "user_budget_create")]
pub async fn user_budget_create(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<UserBudgetCreateData>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let new_user_budget: NewUserBudget = data
        .clone()
        .try_into()
        .map_err(NormalApiError::ValidationError)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let username =
        select_user_name_from_db(&mut transaction, data.user as u64).await?;
    let id =
        insert_user_budget_into_db(&mut transaction, &new_user_budget).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let user_budget_created = UserBudget {
        id: id as u32,
        user: new_user_budget.user_id as u32,
        username,
        year: new_user_budget.year,
        amount: new_user_budget.amount as u32,
    };
    Ok(HttpResponse::Created()
        .content_type("application/json")
        .json(user_budget_created))
}
