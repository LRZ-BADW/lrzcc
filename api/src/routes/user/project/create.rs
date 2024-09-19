use actix_web::body::BoxBody;
use actix_web::http::header::HeaderValue;
use actix_web::http::{header::CONTENT_TYPE, StatusCode};
use actix_web::web::{Data, Json, ReqData};
use actix_web::HttpResponse;
use actix_web::ResponseError;
use anyhow::Context;
use lrzcc_wire::error::{error_chain_fmt, ErrorResponse};
use lrzcc_wire::user::{Project, ProjectCreateData, ProjectCreated, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

#[derive(thiserror::Error)]
pub enum ProjectCreateError {
    #[error("{0}")]
    ValidationError(String),
    #[error("{0}")]
    AuthorizationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for ProjectCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ProjectCreateError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let (status_code, message) = match self {
            ProjectCreateError::ValidationError(message) => {
                (StatusCode::BAD_REQUEST, message.clone())
            }
            ProjectCreateError::AuthorizationError(message) => {
                (StatusCode::FORBIDDEN, message.clone())
            }
            ProjectCreateError::UnexpectedError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error, contact admin or check logs"
                    .to_string(),
            ),
        };
        HttpResponse::build(status_code)
            .insert_header((
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            ))
            // TODO: handle unwrap
            .body(
                serde_json::to_string(&ErrorResponse { detail: message })
                    .unwrap(),
            )
    }
}

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

impl From<InsertProjectError> for ProjectCreateError {
    fn from(value: InsertProjectError) -> Self {
        match value {
            InsertProjectError::ValidationError(message) => {
                ProjectCreateError::ValidationError(message)
            }
            InsertProjectError::UnexpectedError(error) => {
                ProjectCreateError::UnexpectedError(error)
            }
        }
    }
}

#[tracing::instrument(name = "project_create")]
pub async fn project_create(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    data: Json<ProjectCreateData>,
) -> Result<HttpResponse, ProjectCreateError> {
    if !user.is_staff {
        return Err(ProjectCreateError::AuthorizationError(
            "Admin privileges required".to_string(),
        ));
    }
    let new_project: NewProject = data
        .0
        .try_into()
        .map_err(ProjectCreateError::ValidationError)?;
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

#[derive(thiserror::Error)]
pub enum InsertProjectError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for InsertProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[tracing::instrument(
    name = "Insert new project into database",
    skip(new_project, transaction)
)]
pub async fn insert_project(
    transaction: &mut Transaction<'_, MySql>,
    new_project: &NewProject,
) -> Result<u64, InsertProjectError> {
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
        return Err(InsertProjectError::ValidationError(
            "Failed to insert new project, a conflicting entry exists"
                .to_string(),
        ));
    }
    let id = result.last_insert_id();
    Ok(id)
}
