use crate::error::OptionApiError;
use actix_web::web::{Data, Query, ReqData};
use actix_web::HttpResponse;
use lrzcc_wire::accounting::ServerCostParams;
use sqlx::MySqlPool;

use lrzcc_wire::user::{Project, User};
#[tracing::instrument(name = "server_consumption")]
pub async fn server_cost(
    user: ReqData<User>,
    // TODO: not necessary?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<ServerCostParams>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    todo!()
}
