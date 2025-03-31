use actix_web::{
    web::{get, scope},
    Scope,
};

pub mod project;
use project::projects_scope;
#[allow(clippy::module_inception)]
pub mod user;
use user::users_scope;
mod me;
use me::user_me;

pub fn user_scope() -> Scope {
    scope("/user")
        .service(projects_scope())
        .service(users_scope())
        .route("/me", get().to(user_me))
}
