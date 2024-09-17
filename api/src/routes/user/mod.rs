use actix_web::web::scope;
use actix_web::Scope;

mod project;
use project::projects_scope;

pub fn user_scope() -> Scope {
    scope("/user").service(projects_scope())
}
