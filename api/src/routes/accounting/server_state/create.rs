use crate::authorization::require_admin_user;
use crate::database::{
    resources::flavor::select_flavor_name_from_db,
    user::user::select_user_name_from_db,
};
use crate::error::{MinimalApiError, NormalApiError, OptionApiError};
use actix_web::web::{Data, Json, ReqData};
use actix_web::HttpResponse;
use anyhow::Context;
use chrono::{DateTime, Utc};
use lrzcc_wire::accounting::{ServerState, ServerStateCreateData};
use lrzcc_wire::user::{Project, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

pub struct NewServerState {
    pub begin: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub instance_id: String, // UUIDv4
    pub instance_name: String,
    pub flavor: u32,
    // TODO we need an enum here
    pub status: String,
    pub user: u32,
}

// TODO really validate data
impl TryFrom<ServerStateCreateData> for NewServerState {
    type Error = String;

    fn try_from(data: ServerStateCreateData) -> Result<Self, Self::Error> {
        Ok(Self {
            begin: data.begin.to_utc(),
            end: data.end.map(|d| d.to_utc()),
            instance_id: data.instance_id,
            instance_name: data.instance_name,
            flavor: data.flavor,
            status: data.status,
            user: data.user,
        })
    }
}

#[tracing::instrument(name = "server_state_create")]
pub async fn server_state_create(
    user: ReqData<User>,
    // TODO: we don't need this right?
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    data: Json<ServerStateCreateData>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let new_server_state: NewServerState = data
        .clone()
        .try_into()
        .map_err(NormalApiError::ValidationError)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let username = select_user_name_from_db(
        &mut transaction,
        new_server_state.user as u64,
    )
    .await?;
    let flavor_name = select_flavor_name_from_db(
        &mut transaction,
        new_server_state.flavor as u64,
    )
    .await?;
    let id = insert_server_state_into_db(&mut transaction, &new_server_state)
        .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let server_state_created = ServerState {
        id: id as u32,
        begin: data.begin,
        end: data.end,
        instance_id: new_server_state.instance_id.clone(),
        instance_name: new_server_state.instance_name.clone(),
        flavor: new_server_state.flavor,
        flavor_name,
        status: new_server_state.status.clone(),
        user: new_server_state.user,
        username,
    };
    Ok(HttpResponse::Created()
        .content_type("application/json")
        .json(server_state_created))
}

#[tracing::instrument(
    name = "insert_server_state_into_db",
    skip(new_server_state, transaction)
)]
pub async fn insert_server_state_into_db(
    transaction: &mut Transaction<'_, MySql>,
    new_server_state: &NewServerState,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query1 = sqlx::query!(
        r#"
        INSERT IGNORE INTO accounting_state (begin, end)
        VALUES (?, ?)
        "#,
        new_server_state.begin,
        new_server_state.end,
    );
    let result1 = transaction
        .execute(query1)
        .await
        .context("Failed to execute insert query")?;
    if result1.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new state, a conflicting entry exists"
                .to_string(),
        ));
    }
    let id = result1.last_insert_id();
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query2 = sqlx::query!(
        r#"
        INSERT IGNORE INTO accounting_serverstate (
            state_ptr_id, instance_id, instance_name, status, flavor_id, user_id
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        id,
        new_server_state.instance_id,
        new_server_state.instance_name,
        new_server_state.status,
        new_server_state.flavor,
        new_server_state.user
    );
    let result2 = transaction
        .execute(query2)
        .await
        .context("Failed to execute insert query")?;
    if result2.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new server state, a conflicting entry exists"
                .to_string(),
        ));
    }
    Ok(id)
}
