use super::get::select_project_from_db;
use crate::error::{
    require_admin_user, NotFoundOrUnexpectedApiError, OptionApiError,
};
use actix_web::web::{Data, Json, Path, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{Project, ProjectModifyData, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::ProjectIdParam;

#[tracing::instrument(name = "project_modify")]
pub async fn project_modify(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    data: Json<ProjectModifyData>,
    params: Path<ProjectIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    // TODO: do further validation
    if data.id != params.project_id {
        return Err(OptionApiError::ValidationError(
            "ID in URL does not match ID in body".to_string(),
        ));
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let project = update_project_in_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(project))
}

#[tracing::instrument(name = "update_project_in_db", skip(data, transaction))]
pub async fn update_project_in_db(
    transaction: &mut Transaction<'_, MySql>,
    data: &ProjectModifyData,
) -> Result<Project, NotFoundOrUnexpectedApiError> {
    let row = select_project_from_db(transaction, data.id as u64).await?;
    let name = data.name.clone().unwrap_or(row.name);
    let openstack_id = data.openstack_id.clone().unwrap_or(row.openstack_id);
    let user_class = data.user_class.unwrap_or(row.user_class);
    let query = sqlx::query!(
        r#"
        UPDATE user_project
        SET name = ?, openstack_id = ?, user_class = ?
        WHERE id = ?
        "#,
        name,
        openstack_id,
        user_class,
        data.id,
    );
    transaction
        .execute(query)
        .await
        .context("Failed to execute update query")?;
    let project = Project {
        id: data.id,
        name,
        openstack_id,
        user_class,
    };
    Ok(project)
}
