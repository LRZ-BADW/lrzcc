use crate::authorization::{
    require_admin_user, require_master_user,
    require_master_user_or_return_not_found,
};
use crate::database::accounting::server_state::{
    select_all_server_states_from_db, select_server_states_by_project_from_db,
    select_server_states_by_server_from_db,
    select_server_states_by_user_from_db,
};
use crate::database::user::user::select_user_from_db;
use crate::error::OptionApiError;
use actix_web::web::{Data, Query, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::accounting::ServerStateListParams;
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

#[tracing::instrument(name = "server_state_list")]
pub async fn server_state_list(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<ServerStateListParams>,
) -> Result<HttpResponse, OptionApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let server_states = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        select_all_server_states_from_db(&mut transaction).await?
    } else if let Some(project_id) = params.project {
        require_master_user(&user, project_id)?;
        select_server_states_by_project_from_db(
            &mut transaction,
            project_id as u64,
        )
        .await?
    } else if let Some(user_id) = params.user {
        let user1 = select_user_from_db(&mut transaction, user_id as u64)
            .await
            .context("Failed to select user")?;
        require_master_user_or_return_not_found(&user, user1.project)?;
        select_server_states_by_user_from_db(&mut transaction, user1.id as u64)
            .await?
    } else if let Some(server_id) = params.server.clone() {
        // TODO: can we make this master user accessible?
        require_admin_user(&user)?;
        select_server_states_by_server_from_db(&mut transaction, server_id)
            .await?
    } else {
        select_server_states_by_user_from_db(&mut transaction, user.id as u64)
            .await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(server_states))
}
