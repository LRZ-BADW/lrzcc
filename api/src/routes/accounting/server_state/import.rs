use actix_web::{
    web::{Data, ReqData},
    HttpResponse,
};
use lrzcc_wire::user::User;
use sqlx::MySqlPool;

use crate::error::UnexpectedOnlyError;

#[tracing::instrument(name = "server_state_import")]
pub async fn server_state_import(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
) -> Result<HttpResponse, UnexpectedOnlyError> {
    todo!()
}
