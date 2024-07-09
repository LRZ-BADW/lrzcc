use crate::common::{print_object_list, print_single_object, Execute, Format};
use chrono::{DateTime, Utc};
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

    #[clap(about = "Create a new server state")]
    Create {
        #[clap(help = "Begin of the server state")]
        begin: DateTime<Utc>,

        #[clap(help = "UUIDv4 of the instance")]
        instance_id: String, // UUIDv4

        #[clap(help = "Name of the instance")]
        instance_name: String,

        // TODO validate this
        #[clap(help = "ID of the flavor")]
        flavor: u32,

        // TODO need some enum of choices here
        #[clap(help = "Status of the instance")]
        status: String,

        // TODO validate this
        #[clap(help = "ID of the user")]
        user: u32,

        #[clap(help = "End of the server state")]
        end: Option<DateTime<Utc>>,
    },

    #[clap(about = "Modify a server state")]
    Modify {
        #[clap(help = "ID of the server state")]
        id: u32,

        #[clap(long, short, help = "Begin of the server state")]
        begin: Option<DateTime<Utc>>,

        #[clap(long, short, help = "End of the server state")]
        end: Option<DateTime<Utc>>,

        #[clap(
            long,
            short,
            help = "ID of the instance the server state belongs to"
        )]
        instance_id: Option<String>,

        #[clap(
            long,
            short = 'I',
            help = "Current name of the instance the server state belongs to"
        )]
        instance_name: Option<String>,

        #[clap(
            long,
            short,
            help = "Current flavor of the instance the server state belongs to"
        )]
        flavor: Option<u32>,

        // TODO we need some enum here
        #[clap(
            long,
            short,
            help = "Current status of the instance the server state belongs to"
        )]
        status: Option<String>,

        #[clap(
            long,
            short,
            help = "ID of the user the instance of the state belongs to"
        )]
        user: Option<u32>,
    },

    #[clap(about = "Delete server state with given ID")]
    Delete { id: u32 },
}
pub(crate) use ServerStateCommand::*;

impl Execute for ServerStateCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            List { filter } => list(api, format, filter),
            Get { id } => get(api, format, id),
            Create {
                begin,
                end,
                instance_id,
                instance_name,
                flavor,
                status,
                user,
            } => create(
                api,
                format,
                *begin,
                *end,
                instance_id.clone(),
                instance_name.clone(),
                *flavor,
                status.clone(),
                *user,
            ),
            Modify {
                id,
                begin,
                end,
                instance_id,
                instance_name,
                flavor,
                status,
                user,
            } => modify(
                api,
                format,
                *id,
                *begin,
                *end,
                instance_id.clone(),
                instance_name.clone(),
                *flavor,
                status.clone(),
                *user,
            ),
            Delete { id } => delete(api, id),
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

#[allow(clippy::too_many_arguments)]
fn create(
    api: lrzcc::Api,
    format: Format,
    begin: DateTime<Utc>,
    end: Option<DateTime<Utc>>,
    instance_id: String, // UUIDv4
    instance_name: String,
    flavor: u32,
    status: String,
    user: u32,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.server_state.create(
        begin,
        instance_id,
        instance_name,
        flavor,
        status,
        user,
    );
    if let Some(end) = end {
        request.end(end);
    }
    print_single_object(request.send()?, format)
}

#[allow(clippy::too_many_arguments)]
fn modify(
    api: lrzcc::Api,
    format: Format,
    id: u32,
    begin: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    instance_id: Option<String>,
    instance_name: Option<String>,
    flavor: Option<u32>,
    status: Option<String>,
    user: Option<u32>,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.server_state.modify(id);
    if let Some(begin) = begin {
        request.begin(begin);
    }
    if let Some(end) = end {
        request.end(end);
    }
    if let Some(instance_id) = instance_id {
        request.instance_id(instance_id);
    }
    if let Some(instance_name) = instance_name {
        request.instance_name(instance_name);
    }
    if let Some(flavor) = flavor {
        request.flavor(flavor);
    }
    if let Some(status) = status {
        request.status(status);
    }
    if let Some(user) = user {
        request.user(user);
    }
    print_single_object(request.send()?, format)
}

fn delete(api: lrzcc::Api, id: &u32) -> Result<(), Box<dyn Error>> {
    // TODO dangerous operations like this one should be protected by a
    // confirmation prompt
    Ok(api.server_state.delete(*id)?)
}
