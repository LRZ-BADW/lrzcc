use actix_web::web::{
    delete,
    // get, patch, post,
    scope,
};
use actix_web::Scope;
use serde::Deserialize;

// mod create;
// use create::flavor_group_create;
// mod list;
// use list::flavor_group_list;
// mod get;
// use get::flavor_group_get;
// mod modify;
// use modify::flavor_group_modify;
mod delete;
use delete::flavor_group_delete;

pub fn flavor_groups_scope() -> Scope {
    scope("/serverstates")
        // .route("/", post().to(flavor_group_create))
        // .route("", get().to(flavor_group_list))
        // .route("/{flavor_group_id}", get().to(flavor_group_get))
        // TODO: what about PUT?
        // .route("/{flavor_group_id}/", patch().to(flavor_group_modify))
        .route("/{flavor_group_id}/", delete().to(flavor_group_delete))
}

// TODO: wouldn't a general IdParam be better?
#[derive(Deserialize, Debug)]
struct FlavorGroupIdParam {
    // TODO: why is this necessary, when this is clearly read in query_as
    #[allow(unused)]
    flavor_group_id: u32,
}
