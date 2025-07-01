use actix_web::{Scope, web::scope};

mod flavor_price;
use flavor_price::flavor_prices_scope;

pub fn pricing_scope() -> Scope {
    scope("/pricing").service(flavor_prices_scope())
}
