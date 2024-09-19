use crate::error::not_found_error;
use actix_web::web::{Data, Path, ReqData};
use actix_web::HttpResponse;
use lrzcc_wire::user::{Project, User};
use sqlx::MySqlPool;

use super::ProjectIdParam;

#[tracing::instrument(name = "project_delete")]
pub async fn project_delete(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<ProjectIdParam>,
) -> Result<HttpResponse, actix_web::Error> {
    if sqlx::query!(
        r#"DELETE FROM user_project WHERE id = ?"#,
        params.project_id,
    )
    .execute(db_pool.get_ref())
    .await
    .is_err()
    {
        // TODO there might be other errors as well
        // TODO apply context and map_err
        return Err(not_found_error("Project with given ID not found"));
    };

    Ok(HttpResponse::NoContent().finish())
}
