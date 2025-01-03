use actix_web::web::scope;
use actix_web::Scope;

mod server_state;
use server_state::server_states_scope;
mod server_consumption;
use server_consumption::server_consumption_scope;

pub fn accounting_scope() -> Scope {
    scope("/accounting")
        .service(server_states_scope())
        .service(server_consumption_scope())
}
