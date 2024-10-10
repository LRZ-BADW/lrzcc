use actix_web::web::scope;
use actix_web::Scope;

mod flavor_quota;
use flavor_quota::flavor_quotas_scope;

pub fn quota_scope() -> Scope {
    scope("/quota").service(flavor_quotas_scope())
}
