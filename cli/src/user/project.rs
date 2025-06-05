use std::error::Error;

use anyhow::{anyhow, Context};
use avina_wire::user::ProjectRetrieved;
use clap::{Args, Subcommand};

use crate::common::{
    ask_for_confirmation, print_object_list, print_single_object, Execute,
    Format,
};

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct ProjectListFilter {
    #[clap(short, long, help = "Display all projects", action)]
    all: bool,

    #[clap(short, long, help = "Display projects of given user class")]
    // TODO: use enum for this
    user_class: Option<u32>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum ProjectCommand {
    #[clap(about = "List projects")]
    List {
        #[clap(flatten)]
        filter: ProjectListFilter,
    },

    #[clap(
        visible_alias = "show",
        about = "Show project with given name, ID, or openstack ID"
    )]
    Get { name_or_id: String },

    #[clap(about = "Create a new project")]
    Create {
        #[clap(help = "Name of the project")]
        name: String,

        #[clap(help = "Openstack UUIDv4 of the project")]
        openstack_id: String,

        // TODO we need some enum here
        #[clap(
            long,
            short,
            help = "User class of the project (0,1,2,3,4,5,6)"
        )]
        user_class: Option<u32>,
    },

    #[clap(about = "Modify a project")]
    Modify {
        // TODO use name OpenStack consistently
        #[clap(help = "Name, ID, or openstack ID of the project")]
        name_or_id: String,

        #[clap(long, short, help = "Name of the project")]
        name: Option<String>,

        #[clap(long, short, help = "Openstack UUIDv4 of the project")]
        openstack_id: Option<String>,

        // TODO we need some enum here
        #[clap(
            long,
            short,
            help = "User class of the project (0,1,2,3,4,5,6)"
        )]
        user_class: Option<u32>,
    },

    #[clap(about = "Delete project with given name, ID or OpenStack ID")]
    Delete { name_or_id: String },
}
pub(crate) use ProjectCommand::*;

impl Execute for ProjectCommand {
    fn execute(
        &self,
        api: avina::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            List { filter } => list(api, format, filter),
            Get { name_or_id } => get(api, format, name_or_id),
            Create {
                name,
                openstack_id,
                user_class,
            } => create(
                api,
                format,
                name.to_owned(),
                openstack_id.to_owned(),
                *user_class,
            ),
            Modify {
                name_or_id,
                name,
                openstack_id,
                user_class,
            } => modify(
                api,
                format,
                name_or_id,
                name.to_owned(),
                openstack_id.to_owned(),
                *user_class,
            ),
            Delete { name_or_id } => delete(api, name_or_id),
        }
    }
}

fn list(
    api: avina::Api,
    format: Format,
    filter: &ProjectListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.project.list();
    if filter.all {
        request.all();
    } else if let Some(user_class) = filter.user_class {
        request.user_class(user_class);
    }
    print_object_list(request.send()?, format)
}

fn get(
    api: avina::Api,
    format: Format,
    name_or_id: &str,
) -> Result<(), Box<dyn Error>> {
    let id = find_id(&api, name_or_id)?;
    match api.project.get(id)? {
        ProjectRetrieved::Normal(project) => {
            print_single_object(project, format)?
        }
        ProjectRetrieved::Detailed(project) => {
            print_single_object(project, format)?
        }
    };
    Ok(())
}

// TODO something here doesn't work ... no idea why so far
fn create(
    api: avina::Api,
    format: Format,
    name: String,
    openstack_id: String,
    user_class: Option<u32>,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.project.create(name, openstack_id);
    if let Some(user_class) = user_class {
        request.user_class(user_class);
    }
    print_single_object(request.send()?, format)
}

#[allow(clippy::too_many_arguments)]
fn modify(
    api: avina::Api,
    format: Format,
    name_or_id: &str,
    name: Option<String>,
    openstack_id: Option<String>,
    user_class: Option<u32>,
) -> Result<(), Box<dyn Error>> {
    let id = find_id(&api, name_or_id)?;
    let mut request = api.project.modify(id);
    if let Some(name) = name {
        request.name(name);
    }
    if let Some(openstack_id) = openstack_id {
        request.openstack_id(openstack_id);
    }
    if let Some(user_class) = user_class {
        request.user_class(user_class);
    }
    print_single_object(request.send()?, format)
}

fn delete(api: avina::Api, name_or_id: &str) -> Result<(), Box<dyn Error>> {
    let id = find_id(&api, name_or_id)?;
    ask_for_confirmation()?;
    Ok(api.project.delete(id)?)
}

pub(crate) fn find_id(
    api: &avina::Api,
    name_or_id: &str,
) -> Result<u32, anyhow::Error> {
    if let Ok(id) = name_or_id.parse::<u32>() {
        return Ok(id);
    }
    // TODO cache me across arguments
    let me = api.user.me().context("Failed to get own user")?;
    let mut request = api.project.list();
    if me.is_staff {
        request.all();
    }
    let projects = request.send()?;
    if let Some(project) = projects
        .into_iter()
        .find(|p| p.openstack_id == name_or_id || p.name == name_or_id)
    {
        return Ok(project.id);
    }
    Err(anyhow!(
        "Could not find project with name or openstack ID: {name_or_id}"
    ))
}
