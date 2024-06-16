use crate::common::{print_object_list, Execute, Format};
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct FlavorQuotaListFilter {
    #[clap(short, long, help = "Display all flavor quotas", action)]
    all: bool,

    #[clap(
        short,
        long,
        help = "Display flavor quotas of flavor group with given ID"
    )]
    // TODO validate that this is a valid group ID
    group: Option<u32>,

    #[clap(short, long, help = "Display flavor quotas of user with given ID")]
    // TODO validate that this is a valid user ID
    user: Option<u32>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum FlavorQuotaCommand {
    #[clap(about = "List flavor quotas")]
    List {
        #[clap(flatten)]
        filter: FlavorQuotaListFilter,
    },
}

impl Execute for FlavorQuotaCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            FlavorQuotaCommand::List { filter } => list(api, format, filter),
        }
    }
}

fn list(
    api: lrzcc::Api,
    format: Format,
    filter: &FlavorQuotaListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.flavor_quota.list();
    if filter.all {
        request.all();
    } else if let Some(group) = filter.group {
        request.group(group);
    } else if let Some(user) = filter.user {
        request.user(user);
    }
    print_object_list(request.send()?, format)
}
