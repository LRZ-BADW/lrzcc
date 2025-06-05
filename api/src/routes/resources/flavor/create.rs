use actix_web::{
    web::{Data, Json, ReqData},
    HttpResponse,
};
use anyhow::Context;
use avina_wire::{
    resources::{FlavorCreateData, FlavorDetailed, FlavorGroupMinimal},
    user::{Project, User},
};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::resources::{
        flavor::insert_flavor_into_db,
        flavor_group::select_flavor_group_name_from_db,
    },
    error::OptionApiError,
};

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
