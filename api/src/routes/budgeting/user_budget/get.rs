use super::UserBudgetIdParam;
use crate::database::budgeting::user_budget::select_user_budget_from_db;
use crate::database::user::user::select_user_from_db;
use crate::error::AuthOnlyError;
use crate::error::OptionApiError;
use actix_web::web::{Data, Path, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

#[tracing::instrument(name = "user_budget_get")]
pub async fn user_budget_get(
    user: ReqData<User>,
    // TODO: not necessary?
    project: ReqData<Project>,
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
    #[allow(clippy::nonminimal_bool)]
    if !user.is_staff
        && !(user.role == 1 && user.id == user_budget.user)
        && !(user.role == 2 && user.project == user_budget_user.project)
    {
        return Err(AuthOnlyError::AuthorizationError(
            "Admin privileges, master user of project or respective user required"
                .to_string(),
        ).into());
    }
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(user_budget))
}
