use actix_web::{
    web::{Data, ReqData},
    HttpResponse,
};
use anyhow::Context;
use lrzcc_wire::{accounting::ServerStateImport, user::User};
use sqlx::MySqlPool;
use std::collections::HashMap;
use std::hash::Hash;

use crate::{
    authorization::require_admin_user,
    database::accounting::server_state::select_unfinished_server_states_from_db,
    error::NormalApiError, openstack::OpenStack,
};

// NOTE: the hashmap cannot contain (None, None).
fn union_hash_zip<K, V, W>(
    hm1: HashMap<K, V>,
    hm2: HashMap<K, W>,
) -> HashMap<K, (Option<V>, Option<W>)>
where
    K: Clone + Hash + Eq,
    V: Clone + Sized,
    W: Clone + Sized,
{
    let mut hm3 = HashMap::new();
    for (k, v) in hm1.iter() {
        hm3.insert(k.clone(), (Some(v.clone()), None));
    }
    for (k, w) in hm2.iter() {
        hm3.entry(k.clone())
            .and_modify(|(_, u)| *u = Some(w.clone()))
            .or_insert((None, Some(w.clone())));
    }
    hm3
}

#[tracing::instrument(name = "server_state_import", skip(openstack))]
pub async fn server_state_import(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    openstack: Data<OpenStack>,
    // TODO: is the NormalApiError::ValidationError used?
    // Maybe we need a AuthOrUnexpectedError type.
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;

    // TODO: should we add additional context to the error here?
    let servers = openstack
        .get_servers()
        .await?
        .iter()
        .cloned()
        .map(|s| (s.id.clone(), s))
        .collect::<HashMap<_, _>>();
    let states = select_unfinished_server_states_from_db(&mut transaction)
        .await?
        .iter()
        .cloned()
        .map(|s| (s.instance_id.clone(), s))
        .collect::<HashMap<_, _>>();

    let servers_and_states = union_hash_zip(servers, states);

    let new_state_count = 0;
    let end_state_count = 0;

    for server_and_state in servers_and_states.values() {
        match server_and_state {
            (Some(_server), Some(_state)) => {
                // TODO: if the status is unequal, end old and create new state
            }
            (Some(_server), None) => {
                // TODO: create a new state
            }
            (None, Some(_state)) => {
                // TODO: end the state
            }
            (None, None) => {
                // TODO: this cannot happen, print error
            }
        }
    }

    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok().content_type("application/json").json(
        ServerStateImport {
            new_state_count,
            end_state_count,
        },
    ))
}
