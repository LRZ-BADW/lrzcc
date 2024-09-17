use crate::authentication::require_admin_user;
use crate::error::not_found;
use actix_web::middleware::from_fn;
use actix_web::web::{delete, get, patch, post, scope};
use actix_web::Scope;

pub fn projects_scope() -> Scope {
    scope("/projects").service(
        scope("")
            .wrap(from_fn(require_admin_user))
            .route("", post().to(not_found))
            .route("", get().to(not_found))
            .route("", patch().to(not_found))
            .route("", delete().to(not_found)),
    )
}
