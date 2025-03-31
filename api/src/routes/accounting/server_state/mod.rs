use actix_web::{
    web::{delete, get, patch, post, scope},
    Scope,
};
use serde::Deserialize;

mod create;
use create::server_state_create;
mod list;
use list::server_state_list;
mod get;
use get::server_state_get;
mod modify;
use modify::server_state_modify;
mod delete;
use delete::server_state_delete;

pub fn server_states_scope() -> Scope {
    scope("/serverstates")
        .route("/", post().to(server_state_create))
        .route("", get().to(server_state_list))
        .route("/{server_state_id}", get().to(server_state_get))
        // TODO: what about PUT?
        .route("/{server_state_id}/", patch().to(server_state_modify))
        .route("/{server_state_id}/", delete().to(server_state_delete))
}

// TODO: wouldn't a general IdParam be better?
#[derive(Deserialize, Debug)]
struct ServerStateIdParam {
    // TODO: why is this necessary, when this is clearly read in query_as
    #[allow(unused)]
    server_state_id: u32,
}
