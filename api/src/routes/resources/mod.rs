use actix_web::{Scope, web::scope};

mod flavor_group;
use flavor_group::flavor_groups_scope;
mod flavor;
use flavor::flavors_scope;

pub fn resources_scope() -> Scope {
    scope("/resources")
        .service(flavor_groups_scope())
        .service(flavors_scope())
}
