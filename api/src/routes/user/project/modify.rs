use crate::error::{bad_request_error, internal_server_error, not_found_error};
use actix_web::web::{Data, Json, Path, ReqData};
use actix_web::HttpResponse;
use lrzcc_wire::user::{Project, ProjectModifyData, User};
use sqlx::{Executor, FromRow, MySqlPool};

use super::{ProjectIdParam, ProjectRow};

#[tracing::instrument(name = "project_modify")]
pub async fn project_modify(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    data: Json<ProjectModifyData>,
    params: Path<ProjectIdParam>,
) -> Result<HttpResponse, actix_web::Error> {
    if data.id != params.project_id {
        return Err(bad_request_error("ID in URL does not match ID in body"));
    }
    let Ok(mut transaction) = db_pool.begin().await else {
        // TODO there might be other errors as well
        // TODO apply context and map_err
        return Err(internal_server_error("Failed to start transaction"));
    };
    let query = sqlx::query_as::<_, ProjectRow>(
        r#"
        SELECT
            id,
            name,
            openstack_id,
            user_class
        FROM user_project
        WHERE id = ?
        "#,
    )
    .bind(data.id);
    let Ok(row) = transaction.fetch_one(query).await else {
        // TODO there might be other errors as well
        // TODO apply context and map_err
        return Err(not_found_error("Project with given ID not found"));
    };
    let Ok(row) = ProjectRow::from_row(&row) else {
        return Err(internal_server_error("Failed to parse project row"));
    };
    let name = data.name.clone().unwrap_or(row.name);
    let openstack_id = data.openstack_id.clone().unwrap_or(row.openstack_id);
    let user_class = data.user_class.unwrap_or(row.user_class);
    let query = sqlx::query!(
        r#"
        UPDATE user_project
        SET name = ?, openstack_id = ?, user_class = ?
        WHERE id = ?
        "#,
        name,
        openstack_id,
        user_class,
        data.id,
    );
    match transaction.execute(query).await {
        Ok(_) => (),
        Err(e) => {
            // TODO distinguish different database errors
            // TODO apply context and map_err
            tracing::error!("Failed to update project: {:?}", e);
            return Err(bad_request_error(
                "Failed to insert new project, maybe it already exists",
            ));
        }
    };
    if transaction.commit().await.is_err() {
        // TODO apply context and map_err
        return Err(internal_server_error("Failed to commit transaction"));
    }
    let project = Project {
        id: data.id,
        name,
        openstack_id,
        user_class,
    };
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(project))
}
