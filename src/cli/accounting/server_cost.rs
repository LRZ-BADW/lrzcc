use crate::common::{print_single_object, Format};
use chrono::{DateTime, FixedOffset};
use clap::Args;
use std::error::Error;

#[cfg(not(feature = "user"))]
use crate::common::{find_id as project_find_id, find_id as user_find_id};
#[cfg(feature = "user")]
use crate::user::{
    project::find_id as project_find_id, user::find_id as user_find_id,
};

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct ServerCostFilter {
    #[clap(
        short,
        long,
        help = "Calculate server cost for server with given UUID"
    )]
    // TODO validate that this is a valid server UUIDv4
    server: Option<String>,

    #[clap(
        short,
        long,
        help = "Calculate server cost for user with given name, ID, or OpenStack ID"
    )]
    user: Option<String>,

    #[clap(
        short,
        long,
        help = "Calculate server cost for project with given name, ID, or OpenStack ID"
    )]
    project: Option<String>,

    #[clap(
        short,
        long,
        help = "Calculate server cost for entire cloud",
        action
    )]
    all: bool,
}

pub(crate) fn server_cost(
    api: lrzcc::Api,
    format: Format,
    begin: Option<DateTime<FixedOffset>>,
    end: Option<DateTime<FixedOffset>>,
    filter: ServerCostFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.server_cost.get();
    if let Some(server) = &filter.server {
        request.server(server);
    } else if let Some(user) = &filter.user {
        let user_id = user_find_id(&api, user)?;
        request.user(user_id);
    } else if let Some(project) = &filter.project {
        let project_id = project_find_id(&api, project)?;
        request.project(project_id);
    } else if filter.all {
        request.all();
    }
    if let Some(begin) = begin {
        request.begin(begin);
    }
    if let Some(end) = end {
        request.end(end);
    }
    print_single_object(request.send()?, format)
}
