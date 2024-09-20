use super::ProjectRow;
use crate::error::internal_server_error;
use actix_web::web::{Data, ReqData};
use actix_web::HttpResponse;
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

// TODO proper query set and permissions
#[tracing::instrument(name = "project_list")]
pub async fn project_list(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let Ok(rows) = sqlx::query_as!(
        ProjectRow,
        r#"
        SELECT
            id,
            name,
            openstack_id,
            user_class
        FROM user_project
        "#,
    )
    .fetch_all(db_pool.get_ref())
    .await
    else {
        // TODO there might be other errors as well
        // TODO apply context and map_err
        return Err(internal_server_error("Failed to retrieve projects"));
    };

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
