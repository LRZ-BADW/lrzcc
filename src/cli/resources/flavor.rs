use crate::common::{
    ask_for_confirmation, print_object_list, print_single_object, Execute,
    Format,
};
use crate::resources::flavor_group::find_id as flavor_group_find_id;
use anyhow::{anyhow, Context};
use clap::{Args, Subcommand};
use std::error::Error;

#[cfg(not(feature = "user"))]
use crate::common::{find_id as project_find_id, find_id as user_find_id};
#[cfg(feature = "user")]
use crate::user::{
    project::find_id as project_find_id, user::find_id as user_find_id,
};

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct FlavorListFilter {
    #[clap(short, long, help = "Display all flavors", action)]
    all: bool,

    #[clap(
        short,
        long,
        help = "Display flavors of group with given name or ID"
    )]
    group: Option<String>,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct FlavorUsageFilter {
    #[clap(
        short,
        long,
        help = "Calculate flavor usage for user with given name, ID, or OpenStack ID"
    )]
    user: Option<String>,

    #[clap(
        short,
        long,
        help = "Calculate flavor usage for project with given name, ID, or OpenStack ID"
    )]
    project: Option<String>,

    #[clap(
        short,
        long,
        help = "Calculate flavor usage for entire cloud",
        action
    )]
    all: bool,
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

        #[clap(help = "Name or ID of the group this flavor belongs to")]
        group: Option<String>,

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

        #[clap(
            long,
            short,
            help = "Name or ID of the group this flavor belongs to"
        )]
        group: Option<String>,

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

    #[clap(about = "Import new flavors")]
    Import {
        #[clap(
            long,
            short,
            action,
            help = "Suppress output if nothing is imported"
        )]
        quiet: bool,
    },

    #[cfg(feature = "accounting")]
    #[clap(about = "Server cost command")]
    Usage {
        #[clap(flatten)]
        filter: FlavorUsageFilter,

        #[clap(long, short = 'A', help = "Show aggregated flavor usage")]
        aggregate: bool,
    },
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
                group.to_owned(),
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
                group.to_owned(),
                *no_group,
            ),
            Delete { name_or_id } => delete(api, name_or_id),
            Import { quiet } => import(api, format, *quiet),
            Usage { filter, aggregate } => {
                usage(api, format, filter, *aggregate)
            }
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
    } else if let Some(group) = filter.group.to_owned() {
        let group_id = flavor_group_find_id(&api, &group)?;
        request.group(group_id);
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
    group: Option<String>,
    weight: Option<u32>,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.flavor.create(name, openstack_id);
    if let Some(group) = group {
        let group_id = flavor_group_find_id(&api, &group)?;
        request.group(group_id);
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
    group: Option<String>,
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
        let group_id = flavor_group_find_id(&api, &group)?;
        request.group(group_id);
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

fn import(
    api: lrzcc::Api,
    format: Format,
    quiet: bool,
) -> Result<(), Box<dyn Error>> {
    let result = api.flavor.import()?;
    if !quiet || result.new_flavor_count > 0 {
        return print_single_object(result, format);
    }
    Ok(())
}

fn usage(
    api: lrzcc::Api,
    format: Format,
    filter: &FlavorUsageFilter,
    aggregate: bool,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.flavor.usage();
    if aggregate {
        print_object_list(
            if let Some(user) = filter.user.to_owned() {
                let user_id = user_find_id(&api, &user)?;
                request.user_aggregate(user_id)?
            } else if let Some(project) = filter.project.to_owned() {
                let project_id = project_find_id(&api, &project)?;
                request.project_aggregate(project_id)?
            } else if filter.all {
                request.all_aggregate()?
            } else {
                // TODO this causes a http 500 error
                request.mine_aggregate()?
            },
            format,
        )
    } else {
        print_object_list(
            if let Some(user) = filter.user.to_owned() {
                let user_id = user_find_id(&api, &user)?;
                request.user(user_id)?
            } else if let Some(project) = filter.project.to_owned() {
                let project_id = project_find_id(&api, &project)?;
                request.project(project_id)?
            } else if filter.all {
                request.all()?
            } else {
                request.mine()?
            },
            format,
        )
    }
}

// TODO the find id functions can be condensed into a macro
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
