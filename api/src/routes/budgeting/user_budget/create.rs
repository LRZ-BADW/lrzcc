use crate::authorization::require_admin_user;
use crate::database::user::user::select_user_name_from_db;
use crate::error::{MinimalApiError, NormalApiError, OptionApiError};
use actix_web::web::{Data, Json, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use chrono::{Datelike, Utc};
use lrzcc_wire::budgeting::{UserBudget, UserBudgetCreateData};
use lrzcc_wire::user::{Project, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

pub struct NewUserBudget {
    pub user_id: u64,
    pub year: u32,
    pub amount: i64,
}

impl TryFrom<UserBudgetCreateData> for NewUserBudget {
    type Error = String;

    fn try_from(data: UserBudgetCreateData) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: data.user as u64,
            year: data.year.unwrap_or(Utc::now().year() as u32),
            amount: data.amount.unwrap_or(0),
        })
    }
}

#[tracing::instrument(name = "user_budget_create")]
pub async fn user_budget_create(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
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

#[tracing::instrument(
    name = "insert_user_budget_into_db",
    skip(new_user_budget, transaction)
)]
pub async fn insert_user_budget_into_db(
    transaction: &mut Transaction<'_, MySql>,
    new_user_budget: &NewUserBudget,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query = sqlx::query!(
        r#"
        INSERT IGNORE INTO budgeting_userbudget (year, amount, user_id)
        VALUES (?, ?, ?)
        "#,
        new_user_budget.year,
        new_user_budget.amount,
        new_user_budget.user_id,
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
