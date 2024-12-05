use crate::authorization::require_admin_user;
use crate::database::resources::flavor_group::insert_flavor_group_into_db;
use crate::error::OptionApiError;
use actix_web::web::{Data, Json, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::resources::{FlavorGroup, FlavorGroupCreateData};
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

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
