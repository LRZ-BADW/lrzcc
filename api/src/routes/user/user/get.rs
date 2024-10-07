use super::UserIdParam;
use crate::authorization::require_admin_user;
use crate::error::{
    NotFoundOrUnexpectedApiError, OptionApiError, UnexpectedOnlyError,
};
use actix_web::web::{Data, Path, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{Project, User, UserDetailed};
use sqlx::{Executor, FromRow, MySql, MySqlPool, Transaction};

#[tracing::instrument(name = "user_get")]
pub async fn user_get(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<UserIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    // TODO: handle master user access
    if params.user_id != user.id {
        require_admin_user(&user)?;
    }

    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let user =
        select_user_detail_from_db(&mut transaction, params.user_id as u64)
            .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(user))
}

#[tracing::instrument(
    name = "select_maybe_user_detail_from_db",
    skip(transaction)
)]
pub async fn select_maybe_user_detail_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<Option<UserDetailed>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            user.id AS id,
            user.name AS name,
            user.openstack_id AS openstack_id,
            user.role AS role,
            project.id as project__id,
            project.name AS project__name,
            project.user_class AS project__user_class,
            project.name AS project_name,
            user.is_staff AS is_staff,
            user.is_active AS is_active
        FROM user_user AS user, user_project AS project
        WHERE
            user.project_id = project.id AND
            user.id = ?
        "#,
        user_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            UserDetailed::from_row(&row).context("Failed to parse user row")?,
        ),
        None => None,
    })
}

#[tracing::instrument(name = "select_user_detail_from_db", skip(transaction))]
pub async fn select_user_detail_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<UserDetailed, NotFoundOrUnexpectedApiError> {
    select_maybe_user_detail_from_db(transaction, user_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError(
            "User with given ID or linked project not found".to_string(),
        ))
}

#[tracing::instrument(name = "select_maybe_user_from_db", skip(transaction))]
pub async fn select_maybe_user_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<Option<User>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            user.id AS id,
            user.name AS name,
            user.openstack_id AS openstack_id,
            user.role AS role,
            project.id as project,
            project.name AS project_name,
            user.is_staff AS is_staff,
            user.is_active AS is_active
        FROM user_user AS user, user_project AS project
        WHERE
            user.project_id = project.id AND
            user.id = ?
        "#,
        user_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => {
            Some(User::from_row(&row).context("Failed to parse user row")?)
        }
        None => None,
    })
}

#[tracing::instrument(name = "select_user_from_db", skip(transaction))]
pub async fn select_user_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<User, NotFoundOrUnexpectedApiError> {
    select_maybe_user_from_db(transaction, user_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError(
            "User with given ID not found".to_string(),
        ))
}
