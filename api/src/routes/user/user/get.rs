use super::UserIdParam;
use crate::authorization::require_master_user;
use crate::database::user::user::select_user_detail_from_db;
use crate::error::OptionApiError;
use actix_web::web::{Data, Path, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

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
        // TODO: replace by _not_found variant of function
        require_master_user(&user, user2.project.id)?;
    }

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(user2))
}
