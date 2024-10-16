use crate::authorization::require_admin_user;
use crate::database::resources::flavor_group::select_flavor_group_name_from_db;
use crate::error::{MinimalApiError, OptionApiError};
use actix_web::web::{Data, Json, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::resources::{
    FlavorCreateData, FlavorDetailed, FlavorGroupMinimal,
};
use lrzcc_wire::user::{Project, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

#[tracing::instrument(name = "flavor_create")]
pub async fn flavor_create(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    data: Json<FlavorCreateData>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let id = insert_flavor_into_db(&mut transaction, &data).await?;
    let group = if let Some(id) = data.group {
        Some(FlavorGroupMinimal {
            id,
            name: select_flavor_group_name_from_db(&mut transaction, id as u64)
                .await?,
        })
    } else {
        None
    };
    let group_name = group.as_ref().map(|g| g.name.clone());
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let flavor_created = FlavorDetailed {
        id: id as u32,
        name: data.name.clone(),
        openstack_id: data.openstack_id.clone(),
        group,
        group_name,
        weight: data.weight.unwrap_or(0),
    };
    Ok(HttpResponse::Created()
        .content_type("application/json")
        .json(flavor_created))
}

#[tracing::instrument(
    name = "insert_flavor_into_db",
    skip(new_flavor, transaction)
)]
pub async fn insert_flavor_into_db(
    transaction: &mut Transaction<'_, MySql>,
    new_flavor: &FlavorCreateData,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query = sqlx::query!(
        r#"
        INSERT IGNORE INTO resources_flavor (name, openstack_id, weight, group_id)
        VALUES (?, ?, ?, ?)
        "#,
        new_flavor.name,
        new_flavor.openstack_id,
        new_flavor.weight,
        new_flavor.group,
    );
    let result = transaction
        .execute(query)
        .await
        .context("Failed to execute insert query")?;
    // TODO: what about non-existing project_id?
    if result.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new flavor group, a conflicting entry exists"
                .to_string(),
        ));
    }
    let id = result.last_insert_id();
    Ok(id)
}
