use actix_web::{
    web::{Data, Path, ReqData},
    HttpResponse,
};
use anyhow::Context;
use avina_wire::user::{Project, User};
use sqlx::MySqlPool;

use super::FlavorPriceIdParam;
use crate::{
    database::pricing::flavor_price::select_flavor_price_from_db,
    error::OptionApiError,
};

#[tracing::instrument(name = "flavor_price_get")]
pub async fn flavor_price_get(
    user: ReqData<User>,
    // TODO: not necessary?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<FlavorPriceIdParam>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavor_price = select_flavor_price_from_db(
        &mut transaction,
        params.flavor_price_id as u64,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_price))
}
