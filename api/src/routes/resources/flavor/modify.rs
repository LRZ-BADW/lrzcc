use actix_web::{
    web::{Data, Json, Path, ReqData},
    HttpResponse,
};
use anyhow::Context;
use lrzcc_wire::{
    resources::{Flavor, FlavorModifyData},
    user::{Project, User},
};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::FlavorIdParam;
use crate::{
    authorization::require_admin_user,
    database::resources::{
        flavor::select_flavor_from_db,
        flavor_group::select_flavor_group_name_from_db,
    },
    error::{NotFoundOrUnexpectedApiError, OptionApiError},
};

#[tracing::instrument(name = "flavor_modify")]
pub async fn flavor_modify(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    data: Json<FlavorModifyData>,
    params: Path<FlavorIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    // TODO: do further validation
    if data.id != params.flavor_id {
        return Err(OptionApiError::ValidationError(
            "ID in URL does not match ID in body".to_string(),
        ));
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavor = update_flavor_in_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor))
}

#[tracing::instrument(name = "update_flavor_in_db", skip(data, transaction))]
pub async fn update_flavor_in_db(
    transaction: &mut Transaction<'_, MySql>,
    data: &FlavorModifyData,
) -> Result<Flavor, NotFoundOrUnexpectedApiError> {
    let row = select_flavor_from_db(transaction, data.id as u64).await?;
    let name = data.name.clone().unwrap_or(row.name);
    let openstack_id = data.openstack_id.clone().unwrap_or(row.openstack_id);
    let weight = data.weight.unwrap_or(row.weight);
    let group = data.group.unwrap_or(row.group);
    let query = sqlx::query!(
        r#"
        UPDATE resources_flavor
        SET name = ?, openstack_id = ?, weight = ?, group_id = ?
        WHERE id = ?
        "#,
        name,
        openstack_id,
        weight,
        group,
        data.id,
    );
    transaction
        .execute(query)
        .await
        .context("Failed to execute update query")?;
    let group_name = if let Some(group_id) = group {
        Some(
            select_flavor_group_name_from_db(transaction, group_id as u64)
                .await?,
        )
    } else {
        None
    };
    let project = Flavor {
        id: data.id,
        name,
        openstack_id,
        weight,
        group,
        group_name,
    };
    Ok(project)
}
