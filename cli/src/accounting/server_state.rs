use std::error::Error;

use chrono::{DateTime, FixedOffset};
use clap::{Args, Subcommand};

#[cfg(not(feature = "resources"))]
use crate::common::find_id as flavor_find_id;
use crate::common::{
    ask_for_confirmation, print_object_list, print_single_object, Execute,
    Format,
};
#[cfg(not(feature = "user"))]
use crate::common::{find_id as user_find_id, find_id as project_find_id};
#[cfg(feature = "resources")]
use crate::resources::flavor::find_id as flavor_find_id;
#[cfg(feature = "user")]
use crate::user::{
    project::find_id as project_find_id, user::find_id as user_find_id,
};

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

    #[clap(
        short,
        long,
        help = "Display server states of user with given name, ID, or OpenStack ID"
    )]
    user: Option<String>,

    #[clap(
        short,
        long,
        help = "Display server states of project with given name, ID, or OpenStack ID"
    )]
    project: Option<String>,

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

    #[clap(visible_alias = "show", about = "Show server state with given ID")]
    Get { id: u32 },

    #[clap(about = "Create a new server state")]
    Create {
        #[clap(help = "Begin of the server state")]
        begin: DateTime<FixedOffset>,

        #[clap(help = "UUIDv4 of the instance")]
        instance_id: String, // UUIDv4

        #[clap(help = "Name of the instance")]
        instance_name: String,

        #[clap(help = "Name, ID, or OpenStack UUIDv4 of the flavor")]
        flavor: String,

        // TODO need some enum of choices here
        #[clap(help = "Status of the instance")]
        status: String,

        #[clap(help = "Name, ID, or OpenStack ID of the user")]
        user: String,

        #[clap(help = "End of the server state")]
        end: Option<DateTime<FixedOffset>>,
    },

    #[clap(about = "Modify a server state")]
    Modify {
        #[clap(help = "ID of the server state")]
        id: u32,

        #[clap(long, short, help = "Begin of the server state")]
        begin: Option<DateTime<FixedOffset>>,

        #[clap(long, short, help = "End of the server state")]
        end: Option<DateTime<FixedOffset>>,

        #[clap(
            long,
            short,
            help = "OpenStack UUIDv4 of the instance the server state belongs to"
        )]
        // validate that this is a valid UUIDv4
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
            help = "Current flavor of the instance the server state belongs to, given by name, ID, or OpenStack UUIDv4"
        )]
        flavor: Option<String>,

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
            help = "Name, ID, or OpenStack ID of the user the instance of the state belongs to"
        )]
        user: Option<String>,
    },

    #[clap(about = "Delete server state with given ID")]
    Delete { id: u32 },

    #[clap(about = "Import new and end old server states")]
    Import {
        #[clap(
            long,
            short,
            action,
            help = "Suppress output if nothing is imported"
        )]
        quiet: bool,
    },
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
                flavor,
                status.clone(),
                user,
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
                flavor.to_owned(),
                status.clone(),
                user.to_owned(),
            ),
            Delete { id } => delete(api, id),
            Import { quiet } => import(api, format, *quiet),
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
    } else if let Some(user) = &filter.user {
        let user_id = user_find_id(&api, user)?;
        request.user(user_id);
    } else if let Some(project) = &filter.project {
        let project_id = project_find_id(&api, project)?;
        request.project(project_id);
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
    begin: DateTime<FixedOffset>,
    end: Option<DateTime<FixedOffset>>,
    instance_id: String, // UUIDv4
    instance_name: String,
    flavor: &str,
    status: String,
    user: &str,
) -> Result<(), Box<dyn Error>> {
    let flavor_id = flavor_find_id(&api, flavor)?;
    let user_id = user_find_id(&api, user)?;
    ask_for_confirmation()?;
    let mut request = api.server_state.create(
        begin,
        instance_id,
        instance_name,
        flavor_id,
        status,
        user_id,
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
    begin: Option<DateTime<FixedOffset>>,
    end: Option<DateTime<FixedOffset>>,
    instance_id: Option<String>,
    instance_name: Option<String>,
    flavor: Option<String>,
    status: Option<String>,
    user: Option<String>,
) -> Result<(), Box<dyn Error>> {
    ask_for_confirmation()?;
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
        let flavor_id = flavor_find_id(&api, &flavor)?;
        request.flavor(flavor_id);
    }
    if let Some(status) = status {
        request.status(status);
    }
    if let Some(user) = user {
        let user_id = user_find_id(&api, &user)?;
        request.user(user_id);
    }
    print_single_object(request.send()?, format)
}

fn delete(api: lrzcc::Api, id: &u32) -> Result<(), Box<dyn Error>> {
    ask_for_confirmation()?;
    Ok(api.server_state.delete(*id)?)
}

fn import(
    api: lrzcc::Api,
    format: Format,
    quiet: bool,
) -> Result<(), Box<dyn Error>> {
    let result = api.server_state.import()?;
    if !quiet || result.new_state_count > 0 || result.end_state_count > 0 {
        return print_single_object(result, format);
    }
    Ok(())
}
