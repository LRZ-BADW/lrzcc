use actix_web::web::{get, scope};
use actix_web::Scope;

mod get;
use get::server_cost;

pub fn server_cost_scope() -> Scope {
    scope("/servercost").route("/", get().to(server_cost))
}
