use crate::common::{
    ask_for_confirmation, print_object_list, print_single_object, Execute,
    Format,
};
use anyhow::{anyhow, Context};
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

    #[clap(about = "Show user with given name, ID or openstack ID")]
    Get { name_or_id: String },

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

    #[clap(about = "Modify a user")]
    Modify {
        #[clap(help = "ID of the user")]
        id: u32,

        #[clap(long, short, help = "Name of the user")]
        name: Option<String>,

        #[clap(long, short, help = "Openstack UUIDv4 of the user")]
        openstack_id: Option<String>,

        #[clap(long, short, help = "ID of the project this user belongs to")]
        project: Option<u32>,

        // TODO we need some enum here
        #[clap(long, short, help = "Role of the user (1=user, 2=masteruser)")]
        role: Option<u32>,

        #[clap(long, short, help = "Whether the user is an admin")]
        staff: Option<bool>,

        #[clap(long, short, help = "Whether the user is inactive")]
        active: Option<bool>,
    },

    #[clap(about = "Delete user with given ID")]
    Delete { id: u32 },

    #[clap(about = "Show own user")]
    Me,
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
            Get { name_or_id } => get(api, format, name_or_id),
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
            Modify {
                id,
                name,
                openstack_id,
                project,
                role,
                staff,
                active,
            } => modify(
                api,
                format,
                id.to_owned(),
                name.to_owned(),
                openstack_id.to_owned(),
                project.to_owned(),
                role.to_owned(),
                staff.to_owned(),
                active.to_owned(),
            ),
            Delete { id } => delete(api, id),
            Me => me(api, format),
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

fn find_id(api: &lrzcc::Api, name_or_id: &str) -> Result<u32, anyhow::Error> {
    if let Ok(id) = name_or_id.parse::<u32>() {
        return Ok(id);
    }
    // TODO cache me across arguments
    let me = api.user.me().context("Failed to get own user")?;
    let mut request = api.user.list();
    if me.is_staff {
        request.all();
    } else if me.role == 2 {
        request.project(me.project.id);
    }
    let users = request.send()?;
    if let Some(user) = users
        .into_iter()
        .find(|u| u.openstack_id == name_or_id || u.name == name_or_id)
    {
        return Ok(user.id);
    }
    Err(anyhow!(
        "Could not find user with name or openstack ID: {name_or_id}"
    ))
}

fn get(
    api: lrzcc::Api,
    format: Format,
    name_or_id: &str,
) -> Result<(), Box<dyn Error>> {
    let id = find_id(&api, name_or_id)?;
    print_single_object(api.user.get(id)?, format)
}

#[allow(clippy::too_many_arguments)]
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

#[allow(clippy::too_many_arguments)]
fn modify(
    api: lrzcc::Api,
    format: Format,
    id: u32,
    name: Option<String>,
    openstack_id: Option<String>,
    project: Option<u32>,
    role: Option<u32>,
    staff: Option<bool>,
    active: Option<bool>,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.user.modify(id);
    if let Some(name) = name {
        request.name(name);
    }
    if let Some(openstack_id) = openstack_id {
        request.openstack_id(openstack_id);
    }
    if let Some(project) = project {
        request.project(project);
    }
    if let Some(role) = role {
        request.role(role);
    }
    if let Some(staff) = staff {
        request.is_staff(staff);
    }
    if let Some(active) = active {
        request.is_active(active);
    }
    print_single_object(request.send()?, format)
}

fn delete(api: lrzcc::Api, id: &u32) -> Result<(), Box<dyn Error>> {
    ask_for_confirmation()?;
    Ok(api.user.delete(*id)?)
}

fn me(api: lrzcc::Api, format: Format) -> Result<(), Box<dyn Error>> {
    print_single_object(api.user.me()?, format)
}
