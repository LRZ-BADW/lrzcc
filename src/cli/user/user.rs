use crate::common::{print_object_list, print_single_object, Execute, Format};
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct UserListFilter {
    #[clap(short, long, help = "Display all users", action)]
    all: bool,

    #[clap(short, long, help = "Display users of project with given ID")]
    // TODO validate that this is a valid project ID
    project: Option<u32>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum UserCommand {
    #[clap(about = "List users")]
    List {
        #[clap(flatten)]
        filter: UserListFilter,
    },

    #[clap(about = "Show user with given ID")]
    Get { id: u32 },

    #[clap(about = "Create a new user")]
    Create {
        #[clap(help = "Name of the user")]
        name: String,

        #[clap(help = "Openstack UUIDv4 of the user")]
        openstack_id: String,

        #[clap(help = "ID of the project this user belongs to")]
        project: u32,

        // TODO we need some enum here
        #[clap(long, short, help = "Role of the user (1=user, 2=masteruser)")]
        role: Option<u32>,

        #[clap(long, short, help = "Whether the user is an admin", action)]
        staff: bool,

        #[clap(long, short, help = "Whether the user is inactive", action)]
        inactive: bool,
    },

    #[clap(about = "Delete user with given ID")]
    Delete { id: u32 },
}
pub(crate) use UserCommand::*;

impl Execute for UserCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            List { filter } => list(api, format, filter),
            Get { id } => get(api, format, id),
            Create {
                name,
                openstack_id,
                project,
                role,
                staff,
                inactive,
            } => create(
                api,
                format,
                name.to_owned(),
                openstack_id.to_owned(),
                *project,
                *role,
                *staff,
                *inactive,
            ),
            Delete { id } => delete(api, id),
        }
    }
}

fn list(
    api: lrzcc::Api,
    format: Format,
    filter: &UserListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.user.list();
    if filter.all {
        request.all();
    } else if let Some(project) = filter.project {
        request.project(project);
    }
    print_object_list(request.send()?, format)
}

fn get(
    api: lrzcc::Api,
    format: Format,
    id: &u32,
) -> Result<(), Box<dyn Error>> {
    print_single_object(api.user.get(*id)?, format)
}

fn create(
    api: lrzcc::Api,
    format: Format,
    name: String,
    openstack_id: String,
    project: u32,
    role: Option<u32>,
    staff: bool,
    inactive: bool,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.user.create(name, openstack_id, project);
    if let Some(role) = role {
        request.role(role);
    }
    if staff {
        request.staff();
    }
    if inactive {
        request.inactive();
    }
    print_single_object(request.send()?, format)
}

fn delete(api: lrzcc::Api, id: &u32) -> Result<(), Box<dyn Error>> {
    // TODO dangerous operations like this one should be protected by a
    // confirmation prompt
    Ok(api.user.delete(*id)?)
}
