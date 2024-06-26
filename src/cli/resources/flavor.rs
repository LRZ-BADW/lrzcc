use crate::common::{print_object_list, print_single_object, Execute, Format};
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

    #[clap(about = "Show flavor with given ID")]
    Get { id: u32 },

    #[clap(about = "Create a new flavor")]
    Create {
        #[clap(help = "Name of the flavor")]
        name: String,

        #[clap(help = "Openstack UUIDv4 of the flavor")]
        openstack_id: String,

        #[clap(help = "ID of the group this flavor belongs to")]
        group: Option<u32>,

        #[clap(help = "Weight of the flavor within the group")]
        weight: Option<u32>,
    },

    #[clap(about = "Delete flavor with given ID")]
    Delete { id: u32 },
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
            Get { id } => get(api, format, id),
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
            Delete { id } => delete(api, id),
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
    id: &u32,
) -> Result<(), Box<dyn Error>> {
    print_single_object(api.flavor.get(*id)?, format)
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

fn delete(api: lrzcc::Api, id: &u32) -> Result<(), Box<dyn Error>> {
    // TODO dangerous operations like this one should be protected by a
    // confirmation prompt
    Ok(api.flavor.delete(*id)?)
}
