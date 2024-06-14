use crate::common::{print_object_list, Execute, Format};
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
}

impl Execute for FlavorGroupCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            FlavorGroupCommand::List { filter } => list(api, format, filter),
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
