use super::ProjectRow;
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
    let rows = select_all_projects_from_db(&mut transaction).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;

    let projects = rows
        .into_iter()
        .map(|r| Project {
            id: r.id as u32,
            name: r.name,
            openstack_id: r.openstack_id,
            user_class: r.user_class,
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(projects))
}

#[tracing::instrument(name = "select_all_projects_from_db", skip(transaction))]
pub async fn select_all_projects_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<ProjectRow>, UnexpectedOnlyError> {
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
        .map(|r| ProjectRow::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project")?;
    Ok(rows)
}
