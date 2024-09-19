use crate::error::{MinimalApiError, NormalApiError};
use actix_web::web::{Data, Json, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use lrzcc_wire::user::{Project, ProjectCreateData, ProjectCreated, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

pub struct NewProject {
    pub name: String,
    pub openstack_id: String,
    pub user_class: u32,
}

impl TryFrom<ProjectCreateData> for NewProject {
    type Error = String;

    fn try_from(data: ProjectCreateData) -> Result<Self, Self::Error> {
        // TODO really validate data, user_class range, uuid, string length
        Ok(Self {
            name: data.name,
            openstack_id: data.openstack_id,
            user_class: data.user_class.unwrap_or(1),
        })
    }
}

#[tracing::instrument(name = "project_create")]
pub async fn project_create(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    data: Json<ProjectCreateData>,
) -> Result<HttpResponse, NormalApiError> {
    if !user.is_staff {
        return Err(NormalApiError::AuthorizationError(
            "Admin privileges required".to_string(),
        ));
    }
    let new_project: NewProject =
        data.0.try_into().map_err(NormalApiError::ValidationError)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to acquire a database connection from the pool")?;
    let id = insert_project(&mut transaction, &new_project).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let project_created = ProjectCreated {
        id: id as u32,
        name: new_project.name.clone(),
        openstack_id: new_project.openstack_id.clone(),
        user_class: new_project.user_class,
        // TODO retrieve actual values
        users: vec![],
        flavor_groups: vec![],
    };
    Ok(HttpResponse::Created()
        .content_type("application/json")
        .json(project_created))
}

#[tracing::instrument(
    name = "Insert new project into database",
    skip(new_project, transaction)
)]
pub async fn insert_project(
    transaction: &mut Transaction<'_, MySql>,
    new_project: &NewProject,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query = sqlx::query!(
        r#"
        INSERT IGNORE INTO user_project (name, openstack_id, user_class)
        VALUES (?, ?, ?)
        "#,
        new_project.name,
        new_project.openstack_id,
        new_project.user_class
    );
    let result = transaction
        .execute(query)
        .await
        .context("Failed to execute insert query")?;
    if result.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new project, a conflicting entry exists"
                .to_string(),
        ));
    }
    let id = result.last_insert_id();
    Ok(id)
}
