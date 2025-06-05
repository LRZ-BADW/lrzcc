use actix_web::{
    web::{Data, Path, ReqData},
    HttpResponse,
};
use anyhow::Context;
use avina_wire::user::User;
use sqlx::MySqlPool;

use super::ServerStateIdParam;
use crate::{
    authorization::require_master_user_or_return_not_found,
    database::{
        accounting::server_state::select_server_state_from_db,
        user::user::select_user_from_db,
    },
    error::OptionApiError,
};

#[tracing::instrument(name = "server_state_get")]
pub async fn server_state_get(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Path<ServerStateIdParam>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let server_state = select_server_state_from_db(
        &mut transaction,
        params.server_state_id as u64,
    )
    .await?;
    let server_state_user =
        select_user_from_db(&mut transaction, server_state.user as u64).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    if server_state.user != user.id {
        require_master_user_or_return_not_found(
            &user,
            server_state_user.project,
        )?;
    }
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(server_state))
}
