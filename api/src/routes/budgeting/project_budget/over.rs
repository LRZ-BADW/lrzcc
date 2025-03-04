use crate::authorization::require_admin_user;
use crate::error::OptionApiError;
use actix_web::web::{Data, Query, ReqData};
use actix_web::HttpResponse;
use lrzcc_wire::budgeting::ProjectBudgetOverParams;
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

#[tracing::instrument(name = "server_cost")]
pub async fn project_budget_over(
    user: ReqData<User>,
    // TODO: not necessary?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<ProjectBudgetOverParams>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    // TODO: add proper permission check
    require_admin_user(&user)?;
    todo!()
}
