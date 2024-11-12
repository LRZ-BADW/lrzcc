use super::FlavorQuotaIdParam;
use crate::authorization::require_admin_user;
use crate::database::quota::flavor_quota::select_flavor_quota_from_db;
use crate::error::OptionApiError;
use actix_web::web::{Data, Path, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

#[tracing::instrument(name = "flavor_quota_get")]
pub async fn flavor_quota_get(
    user: ReqData<User>,
    // TODO: not necessary?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<FlavorQuotaIdParam>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavor_quota = select_flavor_quota_from_db(
        &mut transaction,
        params.flavor_quota_id as u64,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    if flavor_quota.user != user.id && !user.is_staff {
        return Err(OptionApiError::NotFoundError(
            "Flavor quota not found".to_string(),
        ));
    }
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_quota))
}