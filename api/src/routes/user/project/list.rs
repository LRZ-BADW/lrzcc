use crate::error::{require_admin_user, NormalApiError, UnexpectedOnlyError};
use actix_web::web::{Data, Query, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{Project, ProjectListParams, User};
use sqlx::{Executor, FromRow, MySql, MySqlPool, Transaction};

#[tracing::instrument(name = "project_list")]
pub async fn project_list(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<ProjectListParams>,
) -> Result<HttpResponse, NormalApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let projects = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        select_all_projects_from_db(&mut transaction).await?
    } else if let Some(userclass) = params.userclass {
        require_admin_user(&user)?;
        select_projects_by_userclass_from_db(&mut transaction, userclass as u8)
            .await?
    } else {
        select_projects_by_id_from_db(&mut transaction, project.id as u64)
            .await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(projects))
}

#[tracing::instrument(name = "select_all_projects_from_db", skip(transaction))]
pub async fn select_all_projects_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<Project>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            id,
            name,
            openstack_id,
            user_class
        FROM user_project
        "#,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| Project::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_projects_by_userclass_from_db",
    skip(transaction)
)]
pub async fn select_projects_by_userclass_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_class: u8,
) -> Result<Vec<Project>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            id,
            name,
            openstack_id,
            user_class
        FROM user_project
        where user_class = ?
        "#,
        user_class
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| Project::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project")?;
    Ok(rows)
}

#[tracing::instrument(name = "select_projects_by_id_db", skip(transaction))]
pub async fn select_projects_by_id_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Vec<Project>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            id,
            name,
            openstack_id,
            user_class
        FROM user_project
        WHERE id = ?
        "#,
        project_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| Project::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project")?;
    Ok(rows)
}
