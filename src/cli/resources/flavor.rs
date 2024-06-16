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
}

impl Execute for FlavorCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            FlavorCommand::List { filter } => list(api, format, filter),
            FlavorCommand::Get { id } => get(api, format, id),
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
