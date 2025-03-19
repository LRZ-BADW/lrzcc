use crate::authorization::require_admin_user;
use crate::database::resources::flavor::select_all_flavors_from_db;
use crate::error::NormalApiError;
use actix_web::web::{Data, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::resources::FlavorImport;
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

#[tracing::instrument(name = "flavor_import")]
pub async fn flavor_import(
    user: ReqData<User>,
    // TODO: not necessary?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavors = select_all_flavors_from_db(&mut transaction).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let flavor_import = FlavorImport {
        new_flavor_count: 0,
    };
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_import))
}
