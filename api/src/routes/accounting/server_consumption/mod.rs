use actix_web::{
    Scope,
    web::{get, scope},
};

pub(crate) mod get;
use get::server_consumption;

pub fn server_consumption_scope() -> Scope {
    scope("/serverconsumption").route("/", get().to(server_consumption))
}
