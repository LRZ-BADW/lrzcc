use crate::common::{print_object_list, Execute, Format};
use clap::{Args, Subcommand};
use std::error::Error;

// #[derive(Args, Debug)]
// #[group(multiple = false)]
// pub(crate) struct FlavorListFilter {
//     #[clap(short, long, help = "Display all users", action)]
//     all: bool,
//
//     #[clap(short, long, help = "Display users of project with given ID")]
//     // TODO validate that this is a valid project ID
//     project: Option<u32>,
// }

#[derive(Subcommand, Debug)]
pub(crate) enum FlavorCommand {
    #[clap(about = "List flavors")]
    List, // {
          //     #[clap(flatten)]
          //     filter: FlavorListFilter,
          // },
}

impl Execute for FlavorCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            // FlavorCommand::List { filter } => list(api, format, filter),
            FlavorCommand::List => list(api, format),
        }
    }
}

fn list(
    api: lrzcc::Api,
    format: Format,
    // filter: &FlavorListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.flavor.list();
    // if filter.all {
    //     request.all();
    // } else if let Some(project) = filter.project {
    //     request.project(project);
    // }
    print_object_list(request.send()?, format)
}
