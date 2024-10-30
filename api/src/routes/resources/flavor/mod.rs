use actix_web::web::{
    delete,
    // get,
    patch,
    post,
    scope,
};
use actix_web::Scope;
use serde::Deserialize;

mod create;
use create::flavor_create;
// mod list;
// use list::flavor_list;
// mod get;
// use get::flavor_get;
mod modify;
use modify::flavor_modify;
mod delete;
use delete::flavor_delete;

pub fn flavors_scope() -> Scope {
    scope("/flavors")
        .route("/", post().to(flavor_create))
        // .route("", get().to(flavor_list))
        // .route("/{flavor_id}", get().to(flavor_get))
        // TODO: what about PUT?
        .route("/{flavor_id}/", patch().to(flavor_modify))
        .route("/{flavor_id}/", delete().to(flavor_delete))
}

// TODO: wouldn't a general IdParam be better?
#[derive(Deserialize, Debug)]
struct FlavorIdParam {
    // TODO: why is this necessary, when this is clearly read in query_as
    #[allow(unused)]
    flavor_id: u32,
}
