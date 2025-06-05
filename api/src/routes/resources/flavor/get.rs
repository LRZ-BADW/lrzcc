use actix_web::{
    web::{Data, Path, ReqData},
    HttpResponse,
};
use anyhow::Context;
use avina_wire::user::User;
use sqlx::MySqlPool;

use super::FlavorIdParam;
use crate::{
    authorization::require_admin_user,
    database::resources::flavor::select_flavor_detail_from_db,
    error::OptionApiError,
};

#[tracing::instrument(name = "flavor_get")]
pub async fn flavor_get(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Path<FlavorIdParam>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavor_detail =
        select_flavor_detail_from_db(&mut transaction, params.flavor_id as u64)
            .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_detail))
}
