use crate::common::{print_object_list, print_single_object, Execute, Format};
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct ServerStateListFilter {
    #[clap(
        short,
        long,
        help = "Display server states of server with given UUID"
    )]
    // TODO validate that this is a valid server UUIDv4
    server: Option<String>,

    #[clap(short, long, help = "Display server states of user with given ID")]
    // TODO validate that this is a valid user ID
    user: Option<u32>,

    #[clap(
        short,
        long,
        help = "Display server states of project with given ID"
    )]
    // TODO validate that this is a valid project ID
    project: Option<u32>,

    #[clap(short, long, help = "Display all server states", action)]
    all: bool,
}

#[derive(Subcommand, Debug)]
pub(crate) enum ServerStateCommand {
    #[clap(about = "List server states")]
    List {
        #[clap(flatten)]
        filter: ServerStateListFilter,
    },

    #[clap(about = "Show server state with given ID")]
    Get { id: u32 },
}

impl Execute for ServerStateCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            ServerStateCommand::List { filter } => list(api, format, filter),
            ServerStateCommand::Get { id } => get(api, format, id),
        }
    }
}

fn list(
    api: lrzcc::Api,
    format: Format,
    filter: &ServerStateListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.server_state.list();
    if let Some(server) = &filter.server {
        request.server(server);
    } else if let Some(user) = filter.user {
        request.user(user);
    } else if let Some(project) = filter.project {
        request.project(project);
    } else if filter.all {
        request.all();
    }
    print_object_list(request.send()?, format)
}

fn get(
    api: lrzcc::Api,
    format: Format,
    id: &u32,
) -> Result<(), Box<dyn Error>> {
    print_single_object(api.server_state.get(*id)?, format)
}
