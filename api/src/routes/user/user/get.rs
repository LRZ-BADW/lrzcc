use actix_web::{
    web::{Data, Path, ReqData},
    HttpResponse,
};
use anyhow::Context;
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

use super::UserIdParam;
use crate::{
    authorization::require_master_user_or_return_not_found,
    database::user::user::select_user_detail_from_db, error::OptionApiError,
};

#[tracing::instrument(name = "user_get")]
pub async fn user_get(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<UserIdParam>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let user2 =
        select_user_detail_from_db(&mut transaction, params.user_id as u64)
            .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    if user2.id != user.id {
        require_master_user_or_return_not_found(&user, user2.project.id)?;
    }

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(user2))
}
