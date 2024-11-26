use super::ServerStateIdParam;
use crate::authorization::require_admin_user;
use crate::database::accounting::server_state::select_server_state_from_db;
use crate::error::OptionApiError;
use actix_web::web::{Data, Path, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

#[tracing::instrument(name = "server_state_get")]
pub async fn server_state_get(
    user: ReqData<User>,
    // TODO: not necessary?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<ServerStateIdParam>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let server_state = select_server_state_from_db(
        &mut transaction,
        params.server_state_id as u64,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    if server_state.user != user.id && !user.is_staff {
        return Err(OptionApiError::NotFoundError);
    }
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(server_state))
}
