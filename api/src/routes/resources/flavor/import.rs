use actix_web::{
    HttpResponse,
    web::{Data, ReqData},
};
use anyhow::Context;
use avina_wire::{
    resources::{FlavorCreateData, FlavorImport},
    user::User,
};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::resources::flavor::{
        insert_flavor_into_db, select_all_flavors_from_db,
    },
    error::NormalApiError,
    openstack::OpenStack,
};

#[tracing::instrument(name = "flavor_import", skip(openstack))]
pub async fn flavor_import(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    openstack: Data<OpenStack>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let existing_flavor_names = select_all_flavors_from_db(&mut transaction)
        .await?
        .iter()
        .map(|f| f.name.clone())
        .collect::<Vec<_>>();
    let new_flavors = openstack
        .get_flavors()
        .await?
        .into_iter()
        .filter(|f| !existing_flavor_names.contains(&f.name))
        .collect::<Vec<_>>();
    let new_flavor_count = new_flavors.len() as u32;
    for flavor in new_flavors {
        let data = FlavorCreateData {
            name: flavor.name.clone(),
            openstack_id: flavor.id.clone(),
            group: None,
            weight: None,
        };
        let _ = insert_flavor_into_db(&mut transaction, &data).await?;
    }
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let flavor_import = FlavorImport { new_flavor_count };
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_import))
}
