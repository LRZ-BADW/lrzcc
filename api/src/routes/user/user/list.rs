use crate::authorization::{require_admin_user, require_master_user};
use crate::error::{NormalApiError, UnexpectedOnlyError};
use actix_web::web::{Data, Query, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{Project, User, UserListParams};
use sqlx::{Executor, FromRow, MySql, MySqlPool, Transaction};

#[tracing::instrument(name = "user_list")]
pub async fn user_list(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<UserListParams>,
) -> Result<HttpResponse, NormalApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let users = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        select_all_users_from_db(&mut transaction).await?
    } else if let Some(project_id) = params.project {
        require_master_user(&user, project_id)?;
        select_users_by_project_from_db(&mut transaction, project_id as u64)
            .await?
    } else {
        select_users_by_id_from_db(&mut transaction, user.id as u64).await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(users))
}

#[tracing::instrument(name = "select_all_users_from_db", skip(transaction))]
pub async fn select_all_users_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<User>, UnexpectedOnlyError> {
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
            user.project_id = project.id
        "#,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| User::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_users_by_project_from_db",
    skip(transaction)
)]
pub async fn select_users_by_project_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Vec<User>, UnexpectedOnlyError> {
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
            user.project_id = ?
        "#,
        project_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| User::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project")?;
    Ok(rows)
}

#[tracing::instrument(name = "select_users_by_id_db", skip(transaction))]
pub async fn select_users_by_id_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<Vec<User>, UnexpectedOnlyError> {
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
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| User::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project")?;
    Ok(rows)
}
