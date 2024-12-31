use crate::error::OptionApiError;
use actix_web::web::{Data, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

#[tracing::instrument(name = "server_consumption")]
pub async fn server_consumption(
    user: ReqData<User>,
    // TODO: not necessary?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    let mut _transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    _transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok().content_type("application/json").json(()))
}
