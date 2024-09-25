use crate::error::{require_admin_user, NormalApiError, UnexpectedOnlyError};
use actix_web::web::{Data, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{Project, User};
use sqlx::{Executor, FromRow, MySql, MySqlPool, Transaction};

// TODO proper query set and permissions
#[tracing::instrument(name = "project_list")]
pub async fn project_list(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let projects = select_all_projects_from_db(&mut transaction).await?;
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
