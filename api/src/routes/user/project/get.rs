use super::ProjectIdParam;
use crate::error::{
    require_admin_user, NotFoundOrUnexpectedApiError, OptionApiError,
    UnexpectedOnlyError,
};
use actix_web::web::{Data, Path, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{
    Project, ProjectDetailed, ProjectRetrieved, User, UserMinimal,
};
use sqlx::{Executor, FromRow, MySql, MySqlPool, Transaction};

#[tracing::instrument(name = "project_get")]
pub async fn project_get(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<ProjectIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    if params.project_id != project.id {
        require_admin_user(&user)?;
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let row =
        select_project_from_db(&mut transaction, params.project_id as u64)
            .await?;

    let project = if user.is_staff {
        let users = select_minimal_users_by_project_id_from_db(
            &mut transaction,
            row.id as u64,
        )
        .await?;
        ProjectRetrieved::Detailed(ProjectDetailed {
            id: row.id as u32,
            name: row.name,
            openstack_id: row.openstack_id,
            user_class: row.user_class,
            users,
            flavor_groups: vec![],
        })
    } else {
        ProjectRetrieved::Normal(Project {
            id: row.id as u32,
            name: row.name,
            openstack_id: row.openstack_id,
            user_class: row.user_class,
        })
    };

    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(project))
}

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
    name = "select_minimal_users_by_project_id_from_db",
    skip(transaction)
)]
pub async fn select_minimal_users_by_project_id_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Vec<UserMinimal>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            id,
            name
        FROM user_user
        WHERE project_id = ?
        "#,
        project_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| UserMinimal::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project")?;
    Ok(rows)
}
