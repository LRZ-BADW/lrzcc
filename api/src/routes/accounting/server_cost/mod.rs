use actix_web::{
    Scope,
    web::{get, scope},
};

pub(crate) mod get;
use get::server_cost;

pub fn server_cost_scope() -> Scope {
    scope("/servercost").route("/", get().to(server_cost))
}
