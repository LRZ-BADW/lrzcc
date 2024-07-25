use crate::common::{
    ask_for_confirmation, print_object_list, print_single_object, Execute,
    Format,
};
use anyhow::{anyhow, Context};
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct FlavorListFilter {
    #[clap(short, long, help = "Display all flavors", action)]
    all: bool,

    #[clap(short, long, help = "Display flavors of group with given ID")]
    // TODO validate that this is a valid group ID
    group: Option<u32>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum FlavorCommand {
    #[clap(about = "List flavors")]
    List {
        #[clap(flatten)]
        filter: FlavorListFilter,
    },

    #[clap(about = "Show flavor with given name, ID or OpenStack UUIDv4")]
    Get { name_or_id: String },

    #[clap(about = "Create a new flavor")]
    Create {
        #[clap(help = "Name of the flavor")]
        name: String,

        // TODO verify that this is a UUIDv4
        #[clap(help = "Openstack UUIDv4 of the flavor")]
        openstack_id: String,

        #[clap(help = "ID of the group this flavor belongs to")]
        group: Option<u32>,

        #[clap(help = "Weight of the flavor within the group")]
        weight: Option<u32>,
    },

    #[clap(about = "Modify a flavor")]
    Modify {
        #[clap(help = "Name, ID or OpenStack UUIDv4 of the flavor")]
        name_or_id: String,

        #[clap(long, short, help = "Name of the flavor")]
        name: Option<String>,

        #[clap(long, short, help = "Openstack UUIDv4 of the flavor")]
        openstack_id: Option<String>,

        #[clap(long, short, help = "ID of the group this flavor belongs to")]
        group: Option<u32>,

        #[clap(
            long,
            short = 'G',
            help = "Remove flavor from its group",
            action,
            conflicts_with = "group"
        )]
        no_group: bool,
    },

    #[clap(about = "Delete flavor with given name, ID or OpenStack UUIDv4")]
    Delete { name_or_id: String },
}
pub(crate) use FlavorCommand::*;

impl Execute for FlavorCommand {
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
                group,
                weight,
            } => create(
                api,
                format,
                name.to_owned(),
                openstack_id.to_owned(),
                *group,
                *weight,
            ),
            Modify {
                name_or_id,
                name,
                openstack_id,
                group,
                no_group,
            } => modify(
                api,
                format,
                name_or_id,
                name.clone(),
                openstack_id.clone(),
                *group,
                *no_group,
            ),
            Delete { name_or_id } => delete(api, name_or_id),
        }
    }
}

fn list(
    api: lrzcc::Api,
    format: Format,
    filter: &FlavorListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.flavor.list();
    if filter.all {
        request.all();
    } else if let Some(group) = filter.group {
        request.group(group);
    }
    print_object_list(request.send()?, format)
}

fn get(
    api: lrzcc::Api,
    format: Format,
    name_or_id: &str,
) -> Result<(), Box<dyn Error>> {
    let id = find_id(&api, name_or_id)?;
    print_single_object(api.flavor.get(id)?, format)
}

fn create(
    api: lrzcc::Api,
    format: Format,
    name: String,
    openstack_id: String,
    group: Option<u32>,
    weight: Option<u32>,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.flavor.create(name, openstack_id);
    if let Some(group) = group {
        request.group(group);
    }
    if let Some(weight) = weight {
        request.weight(weight);
    }
    print_single_object(request.send()?, format)
}

fn modify(
    api: lrzcc::Api,
    format: Format,
    name_or_id: &str,
    name: Option<String>,
    openstack_id: Option<String>,
    group: Option<u32>,
    no_group: bool,
) -> Result<(), Box<dyn Error>> {
    let id = find_id(&api, name_or_id)?;
    let mut request = api.flavor.modify(id);
    if let Some(name) = name {
        request.name(name);
    }
    if let Some(openstack_id) = openstack_id {
        request.openstack_id(openstack_id);
    }
    if let Some(group) = group {
        request.group(group);
    } else if no_group {
        request.no_group();
    }
    print_single_object(request.send()?, format)
}

// TODO replace all command functions errors by anyhow::Error
fn delete(api: lrzcc::Api, name_or_id: &str) -> Result<(), Box<dyn Error>> {
    let id = find_id(&api, name_or_id)?;
    ask_for_confirmation()?;
    Ok(api.flavor.delete(id)?)
}

pub(crate) fn find_id(
    api: &lrzcc::Api,
    name_or_id: &str,
) -> Result<u32, anyhow::Error> {
    if let Ok(id) = name_or_id.parse::<u32>() {
        return Ok(id);
    }
    // TODO cache me across arguments
    let me = api.user.me().context("Failed to get own user")?;
    let mut request = api.flavor.list();
    if me.is_staff {
        request.all();
    }
    let projects = request.send()?;
    if let Some(project) = projects
        .into_iter()
        .find(|f| f.openstack_id == name_or_id || f.name == name_or_id)
    {
        return Ok(project.id);
    }
    Err(anyhow!(
        "Could not find flavor with name or openstack ID: {name_or_id}"
    ))
}
