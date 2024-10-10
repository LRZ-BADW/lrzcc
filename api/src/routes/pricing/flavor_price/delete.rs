use crate::authorization::require_admin_user;
use crate::error::{MinimalApiError, NormalApiError};
use actix_web::web::{Data, Path, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{Project, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::FlavorPriceIdParam;

#[tracing::instrument(name = "flavor_price_delete")]
pub async fn flavor_price_delete(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<FlavorPriceIdParam>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    delete_flavor_price_from_db(
        &mut transaction,
        params.flavor_price_id as u64,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::NoContent().finish())
}

#[tracing::instrument(name = "delete_flavor_price_from_db", skip(transaction))]
async fn delete_flavor_price_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_price_id: u64,
) -> Result<(), MinimalApiError> {
    let query = sqlx::query!(
        r#"
        DELETE IGNORE FROM pricing_flavorprice
        WHERE id = ?
        "#,
        flavor_price_id
    );
    let result = transaction
        .execute(query)
        .await
        .context("Failed to execute delete query")?;
    if result.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            // TODO: test that this message is really correct
            "Failed to delete flavor price.".to_string(),
        ));
    }
    Ok(())
}
