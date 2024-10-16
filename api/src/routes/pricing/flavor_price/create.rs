use crate::authorization::require_admin_user;
use crate::database::resources::flavor::select_flavor_name_from_db;
use crate::error::{MinimalApiError, NormalApiError, OptionApiError};
use actix_web::web::{Data, Json, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use chrono::{DateTime, Utc};
use lrzcc_wire::pricing::{FlavorPrice, FlavorPriceCreateData};
use lrzcc_wire::user::{Project, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

pub struct NewFlavorPrice {
    pub flavor_id: u64,
    pub user_class: u32,
    pub unit_price: f64,
    pub start_time: DateTime<Utc>,
}

impl TryFrom<FlavorPriceCreateData> for NewFlavorPrice {
    type Error = String;

    fn try_from(data: FlavorPriceCreateData) -> Result<Self, Self::Error> {
        Ok(Self {
            flavor_id: data.flavor as u64,
            user_class: data.user_class,
            unit_price: data.price.unwrap_or(0.),
            start_time: data
                .start_time
                .map(|d| d.to_utc())
                .unwrap_or(Utc::now()),
        })
    }
}

#[tracing::instrument(name = "flavor_price_create")]
pub async fn flavor_price_create(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
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

#[tracing::instrument(
    name = "insert_flavor_price_into_db",
    skip(new_flavor_price, transaction)
)]
pub async fn insert_flavor_price_into_db(
    transaction: &mut Transaction<'_, MySql>,
    new_flavor_price: &NewFlavorPrice,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query = sqlx::query!(
        r#"
        INSERT IGNORE INTO pricing_flavorprice (user_class, unit_price, start_time, flavor_id)
        VALUES (?, ?, ?, ?)
        "#,
        new_flavor_price.user_class,
        new_flavor_price.unit_price,
        new_flavor_price.start_time,
        new_flavor_price.flavor_id,
    );
    let result = transaction
        .execute(query)
        .await
        .context("Failed to execute insert query")?;
    // TODO: what about non-existing project_id?
    if result.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new flavor price, a conflicting entry exists"
                .to_string(),
        ));
    }
    let id = result.last_insert_id();
    Ok(id)
}
