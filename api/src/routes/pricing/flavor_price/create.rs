use actix_web::{
    HttpResponse,
    web::{Data, Json, ReqData},
};
use anyhow::Context;
use avina_wire::{
    pricing::{FlavorPrice, FlavorPriceCreateData},
    user::User,
};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::{
        pricing::flavor_price::{NewFlavorPrice, insert_flavor_price_into_db},
        resources::flavor::select_flavor_name_from_db,
    },
    error::{NormalApiError, OptionApiError},
};

#[tracing::instrument(name = "flavor_price_create")]
pub async fn flavor_price_create(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<FlavorPriceCreateData>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let new_flavor_price: NewFlavorPrice = data
        .clone()
        .try_into()
        .map_err(NormalApiError::ValidationError)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavor_name = select_flavor_name_from_db(
        &mut transaction,
        new_flavor_price.flavor_id,
    )
    .await?;
    let id = insert_flavor_price_into_db(&mut transaction, &new_flavor_price)
        .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let flavor_price_created = FlavorPrice {
        id: id as u32,
        flavor: new_flavor_price.flavor_id as u32,
        flavor_name,
        user_class: new_flavor_price.user_class,
        unit_price: new_flavor_price.unit_price,
        start_time: new_flavor_price.start_time.fixed_offset(),
    };
    Ok(HttpResponse::Created()
        .content_type("application/json")
        .json(flavor_price_created))
}
