use crate::authorization::require_admin_user;
use crate::database::resources::flavor_group::select_flavor_group_from_db;
use crate::error::{NotFoundOrUnexpectedApiError, OptionApiError};
use actix_web::web::{Data, Json, Path, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::resources::{FlavorGroup, FlavorGroupModifyData};
use lrzcc_wire::user::{Project, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::FlavorGroupIdParam;

#[tracing::instrument(name = "flavor_group_modify")]
pub async fn flavor_group_modify(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    data: Json<FlavorGroupModifyData>,
    params: Path<FlavorGroupIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    // TODO: do further validation
    if data.id != params.flavor_group_id {
        return Err(OptionApiError::ValidationError(
            "ID in URL does not match ID in body".to_string(),
        ));
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavor_group =
        update_flavor_group_in_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_group))
}

#[tracing::instrument(
    name = "update_flavor_group_in_db",
    skip(data, transaction)
)]
pub async fn update_flavor_group_in_db(
    transaction: &mut Transaction<'_, MySql>,
    data: &FlavorGroupModifyData,
) -> Result<FlavorGroup, NotFoundOrUnexpectedApiError> {
    let row = select_flavor_group_from_db(transaction, data.id as u64).await?;
    let name = data.name.clone().unwrap_or(row.name);
    let project = data.project.unwrap_or(row.project);
    let query = sqlx::query!(
        r#"
        UPDATE resources_flavorgroup
        SET name = ?, project_id = ?
        WHERE id = ?
        "#,
        name,
        project,
        data.id,
    );
    transaction
        .execute(query)
        .await
        .context("Failed to execute update query")?;
    let project = FlavorGroup {
        id: data.id,
        name,
        project,
        flavors: row.flavors,
    };
    Ok(project)
}
