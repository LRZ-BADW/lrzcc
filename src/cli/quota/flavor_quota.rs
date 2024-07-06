use crate::common::{print_object_list, print_single_object, Execute, Format};
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

    #[clap(about = "Show flavor quota with given ID")]
    Get { id: u32 },

    #[clap(about = "Create a new flavor quota")]
    Create {
        #[clap(help = "ID of the flavor group")]
        flavor_group: u32,

        #[clap(help = "ID of the user")]
        user: u32,

        #[clap(long, short, help = "Amount of the quota")]
        quota: Option<i64>,
    },

    #[clap(about = "Modify a flavor quota")]
    Modify {
        #[clap(help = "ID of the flavor quota")]
        id: u32,

        #[clap(long, short, help = "User the quota is for")]
        user: Option<u32>,

        #[clap(long, short, help = "Quota amount")]
        quota: Option<i64>,

        #[clap(
            long,
            short,
            help = "ID of the flavor group that should be limited"
        )]
        flavor_group: Option<u32>,
    },

    #[clap(about = "Delete flavor quota with given ID")]
    Delete { id: u32 },
}
pub(crate) use FlavorQuotaCommand::*;

impl Execute for FlavorQuotaCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            List { filter } => list(api, format, filter),
            Get { id } => get(api, format, id),
            Create {
                flavor_group,
                user,
                quota,
            } => create(api, format, *flavor_group, *user, *quota),
            Modify {
                id,
                user,
                quota,
                flavor_group,
            } => modify(api, format, *id, *user, *quota, *flavor_group),
            Delete { id } => delete(api, id),
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

fn get(
    api: lrzcc::Api,
    format: Format,
    id: &u32,
) -> Result<(), Box<dyn Error>> {
    print_single_object(api.flavor_quota.get(*id)?, format)
}

fn create(
    api: lrzcc::Api,
    format: Format,
    flavor_group: u32,
    user: u32,
    quota: Option<i64>,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.flavor_quota.create(flavor_group, user);
    if let Some(quota) = quota {
        request.quota(quota);
    }
    print_single_object(request.send()?, format)
}

fn modify(
    api: lrzcc::Api,
    format: Format,
    id: u32,
    user: Option<u32>,
    quota: Option<i64>,
    flavor_group: Option<u32>,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.flavor_quota.modify(id);
    if let Some(user) = user {
        request.user(user);
    }
    if let Some(quota) = quota {
        request.quota(quota);
    }
    if let Some(flavor_group) = flavor_group {
        request.flavor_group(flavor_group);
    }
    print_single_object(request.send()?, format)
}

fn delete(api: lrzcc::Api, id: &u32) -> Result<(), Box<dyn Error>> {
    Ok(api.flavor_quota.delete(*id)?)
}
