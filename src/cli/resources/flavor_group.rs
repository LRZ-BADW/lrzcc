use crate::common::{
    ask_for_confirmation, print_object_list, print_single_object, Execute,
    Format,
};
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct FlavorGroupListFilter {
    #[clap(short, long, help = "Display all flavors", action)]
    all: bool,
}

#[derive(Subcommand, Debug)]
pub(crate) enum FlavorGroupCommand {
    #[clap(about = "List flavors")]
    List {
        #[clap(flatten)]
        filter: FlavorGroupListFilter,
    },

    #[clap(about = "Show flavor group with given ID")]
    Get { id: u32 },

    #[clap(about = "Create a new flavor group")]
    Create {
        #[clap(help = "Name of the flavor group")]
        name: String,
    },

    #[clap(about = "Modify a flavor group")]
    Modify {
        #[clap(help = "ID of the flavor group")]
        id: u32,

        #[clap(long, short, help = "Name of the flavor group")]
        name: Option<String>,

        #[clap(
            long,
            short,
            help = "ID of the project this flavor group belongs to"
        )]
        project: Option<u32>,
    },

    #[clap(about = "Delete flavor group with given ID")]
    Delete { id: u32 },
}
pub(crate) use FlavorGroupCommand::*;

impl Execute for FlavorGroupCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            List { filter } => list(api, format, filter),
            Get { id } => get(api, format, id),
            Create { name } => create(api, format, name.to_owned()),
            Modify { id, name, project } => {
                modify(api, format, *id, name.to_owned(), project.to_owned())
            }
            Delete { id } => delete(api, id),
        }
    }
}

fn list(
    api: lrzcc::Api,
    format: Format,
    filter: &FlavorGroupListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.flavor_group.list();
    if filter.all {
        request.all();
    }
    print_object_list(request.send()?, format)
}

fn get(
    api: lrzcc::Api,
    format: Format,
    id: &u32,
) -> Result<(), Box<dyn Error>> {
    print_single_object(api.flavor_group.get(*id)?, format)
}

fn create(
    api: lrzcc::Api,
    format: Format,
    name: String,
) -> Result<(), Box<dyn Error>> {
    print_single_object(api.flavor_group.create(name).send()?, format)
}

fn modify(
    api: lrzcc::Api,
    format: Format,
    id: u32,
    name: Option<String>,
    project: Option<u32>,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.flavor_group.modify(id);
    if let Some(name) = name {
        request.name(name);
    }
    if let Some(project) = project {
        request.project(project);
    }
    print_single_object(request.send()?, format)
}

fn delete(api: lrzcc::Api, id: &u32) -> Result<(), Box<dyn Error>> {
    ask_for_confirmation()?;
    Ok(api.flavor_group.delete(*id)?)
}
