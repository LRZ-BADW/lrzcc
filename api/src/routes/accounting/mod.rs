use actix_web::web::scope;
use actix_web::Scope;

mod server_state;
use server_state::server_states_scope;

pub fn accounting_scope() -> Scope {
    scope("/accounting").service(server_states_scope())
}
