use crate::error::bad_request_error;
use actix_web::web::{Data, Json, ReqData};
use actix_web::HttpResponse;
use lrzcc_wire::user::{Project, ProjectCreateData, ProjectCreated, User};
use sqlx::{Executor, MySqlPool};

#[tracing::instrument(name = "project_create")]
pub async fn project_create(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    data: Json<ProjectCreateData>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_class = data.user_class.unwrap_or(1);
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query = sqlx::query!(
        r#"
        INSERT INTO user_project (name, openstack_id, user_class)
        VALUES (?, ?, ?)
        "#,
        data.name,
        data.openstack_id,
        user_class
    );
    let result = match db_pool.execute(query).await {
        Ok(result) => result,
        Err(e) => {
            // TODO distinguish different database errors
            // TODO apply context and map_err
            tracing::error!("Failed to insert new project: {:?}", e);
            return Err(bad_request_error(
                "Failed to insert new project, maybe it already exists",
            ));
        }
    };
    let id = result.last_insert_id();
    let project = ProjectCreated {
        id: id as u32,
        name: data.name.clone(),
        openstack_id: data.openstack_id.clone(),
        user_class,
        // TODO retrieve actual values
        users: vec![],
        flavor_groups: vec![],
    };
    Ok(HttpResponse::Created()
        .content_type("application/json")
        .json(project))
}
