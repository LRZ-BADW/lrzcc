use crate::authorization::require_admin_user;
use crate::error::{MinimalApiError, OptionApiError};
use actix_web::web::{Data, Json, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::resources::{FlavorGroup, FlavorGroupCreateData};
use lrzcc_wire::user::{Project, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

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

#[tracing::instrument(
    name = "insert_flavor_group_into_db",
    skip(new_flavor_group, transaction)
)]
pub async fn insert_flavor_group_into_db(
    transaction: &mut Transaction<'_, MySql>,
    new_flavor_group: &FlavorGroupCreateData,
    project_id: u64,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query = sqlx::query!(
        r#"
        INSERT IGNORE INTO resources_flavorgroup (name, project_id)
        VALUES (?, ?)
        "#,
        new_flavor_group.name,
        project_id,
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
