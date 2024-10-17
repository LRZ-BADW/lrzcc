use crate::error::{NotFoundOrUnexpectedApiError, UnexpectedOnlyError};
use anyhow::Context;
use lrzcc_wire::user::Project;
use sqlx::{Executor, FromRow, MySql, Transaction};

#[tracing::instrument(name = "select_maybe_project_from_db", skip(transaction))]
pub async fn select_maybe_project_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Option<Project>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            id,
            name,
            openstack_id,
            user_class
        FROM user_project AS project
        WHERE
            project.id = ?
        "#,
        project_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            Project::from_row(&row).context("Failed to parse project row")?,
        ),
        None => None,
    })
}

#[tracing::instrument(name = "select_project_from_db", skip(transaction))]
pub async fn select_project_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Project, NotFoundOrUnexpectedApiError> {
    select_maybe_project_from_db(transaction, project_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError(
            "Project with given ID not found".to_string(),
        ))
}

#[tracing::instrument(
    name = "select_maybe_project_name_from_db",
    skip(transaction)
)]
pub async fn select_maybe_project_name_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Option<String>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    #[allow(dead_code)]
    struct Row {
        name: String,
    }
    let query = sqlx::query!(
        r#"
        SELECT name
        FROM user_project AS project
        WHERE
            project.id = ?
        "#,
        project_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            Row::from_row(&row)
                .context("Failed to parse project row")?
                .name,
        ),
        None => None,
    })
}

#[tracing::instrument(name = "select_project_name_from_db", skip(transaction))]
pub async fn select_project_name_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<String, NotFoundOrUnexpectedApiError> {
    select_maybe_project_name_from_db(transaction, project_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError(
            "Project with given ID not found".to_string(),
        ))
}
