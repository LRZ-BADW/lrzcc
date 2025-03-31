use actix_web::{
    web::{Data, Json, ReqData},
    HttpResponse,
};
use anyhow::Context;
use lrzcc_wire::{
    resources::{FlavorGroup, FlavorGroupCreateData},
    user::{Project, User},
};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::resources::flavor_group::insert_flavor_group_into_db,
    error::OptionApiError,
};

#[tracing::instrument(name = "flavor_group_create")]
pub async fn flavor_group_create(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    data: Json<FlavorGroupCreateData>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    // TODO: the project id should be part of the FlavorGroupCreateData
    let name = data.name.clone();
    let id =
        insert_flavor_group_into_db(&mut transaction, &data, project.id as u64)
            .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let flavor_group_created = FlavorGroup {
        id: id as u32,
        name,
        project: project.id,
        flavors: vec![],
    };
    Ok(HttpResponse::Created()
        .content_type("application/json")
        .json(flavor_group_created))
}
