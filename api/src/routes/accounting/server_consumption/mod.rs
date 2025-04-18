use actix_web::{
    web::{get, scope},
    Scope,
};

pub(crate) mod get;
use get::server_consumption;

pub fn server_consumption_scope() -> Scope {
    scope("/serverconsumption").route("/", get().to(server_consumption))
}
