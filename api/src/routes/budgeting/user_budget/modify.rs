use crate::authorization::require_admin_user;
use crate::database::budgeting::user_budget::select_user_budget_from_db;
use crate::error::{NotFoundOrUnexpectedApiError, OptionApiError};
use actix_web::web::{Data, Json, Path, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::budgeting::{UserBudget, UserBudgetModifyData};
use lrzcc_wire::user::{Project, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::UserBudgetIdParam;

#[tracing::instrument(name = "user_budget_modify")]
pub async fn user_budget_modify(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    data: Json<UserBudgetModifyData>,
    params: Path<UserBudgetIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    // TODO: allow master user access
    // TODO: check that cost is below
    // TODO: handle force option
    require_admin_user(&user)?;
    // TODO: do further validation
    if data.id != params.user_budget_id {
        return Err(OptionApiError::ValidationError(
            "ID in URL does not match ID in body".to_string(),
        ));
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let user_budget = update_user_budget_in_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(user_budget))
}

#[tracing::instrument(
    name = "update_user_budget_in_db",
    skip(data, transaction)
)]
pub async fn update_user_budget_in_db(
    transaction: &mut Transaction<'_, MySql>,
    data: &UserBudgetModifyData,
) -> Result<UserBudget, NotFoundOrUnexpectedApiError> {
    let row = select_user_budget_from_db(transaction, data.id as u64).await?;
    let amount = data.amount.unwrap_or(row.amount);
    let query = sqlx::query!(
        r#"
        UPDATE budgeting_userbudget
        SET amount = ?
        WHERE id = ?
        "#,
        amount,
        data.id,
    );
    transaction
        .execute(query)
        .await
        .context("Failed to execute update query")?;
    let project = UserBudget {
        id: data.id,
        amount,
        user: row.user,
        username: row.username,
        year: row.year,
    };
    Ok(project)
}
