use super::get::select_user_from_db;
use crate::authorization::require_admin_user;
use crate::error::{NotFoundOrUnexpectedApiError, OptionApiError};
use actix_web::web::{Data, Json, Path, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{Project, User, UserModifyData};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::UserIdParam;

#[tracing::instrument(name = "user_modify")]
pub async fn user_modify(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    data: Json<UserModifyData>,
    params: Path<UserIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    // TODO: do further validation
    if data.id != params.user_id {
        return Err(OptionApiError::ValidationError(
            "ID in URL does not match ID in body".to_string(),
        ));
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let project = update_user_in_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(project))
}

#[tracing::instrument(name = "update_user_in_db", skip(data, transaction))]
pub async fn update_user_in_db(
    transaction: &mut Transaction<'_, MySql>,
    data: &UserModifyData,
) -> Result<User, NotFoundOrUnexpectedApiError> {
    let row = select_user_from_db(transaction, data.id as u64).await?;
    let name = data.name.clone().unwrap_or(row.name);
    let openstack_id = data.openstack_id.clone().unwrap_or(row.openstack_id);
    let project_id = data.project.unwrap_or(row.project);
    let role = data.role.unwrap_or(row.role);
    let is_staff = data.is_staff.unwrap_or(row.is_staff);
    let is_active = data.is_active.unwrap_or(row.is_active);
    let query = sqlx::query!(
        r#"
        UPDATE user_user
        SET name = ?, openstack_id = ?, project_id = ?, role = ?, is_staff = ?, is_active = ?
        WHERE id = ?
        "#,
        name,
        openstack_id,
        project_id,
        role,
        is_staff,
        is_active,
        data.id,
    );
    transaction
        .execute(query)
        .await
        .context("Failed to execute update query")?;
    let user = User {
        id: data.id,
        name,
        openstack_id,
        project: project_id,
        // TODO: we need to get the new project's name
        project_name: row.project_name,
        role,
        is_staff,
        is_active,
    };
    Ok(user)
}
