use crate::authorization::require_admin_user;
use crate::database::resources::flavor_group::{
    select_all_flavor_groups_from_db, select_lrz_flavor_groups_from_db,
};
use crate::error::NormalApiError;
use actix_web::web::{Data, Query, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::resources::FlavorGroupListParams;
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

#[tracing::instrument(name = "flavor_group_list")]
pub async fn flavor_group_list(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<FlavorGroupListParams>,
) -> Result<HttpResponse, NormalApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavor_groups = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        select_all_flavor_groups_from_db(&mut transaction).await?
    } else {
        select_lrz_flavor_groups_from_db(&mut transaction).await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_groups))
}
