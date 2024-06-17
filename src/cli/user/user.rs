use crate::common::{print_object_list, print_single_object, Execute, Format};
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct UserListFilter {
    #[clap(short, long, help = "Display all users", action)]
    all: bool,

    #[clap(short, long, help = "Display users of project with given ID")]
    // TODO validate that this is a valid project ID
    project: Option<u32>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum UserCommand {
    #[clap(about = "List users")]
    List {
        #[clap(flatten)]
        filter: UserListFilter,
    },

    #[clap(about = "Show user with given ID")]
    Get { id: u32 },
}

impl Execute for UserCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            UserCommand::List { filter } => list(api, format, filter),
            UserCommand::Get { id } => get(api, format, id),
        }
    }
}

fn list(
    api: lrzcc::Api,
    format: Format,
    filter: &UserListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.user.list();
    if filter.all {
        request.all();
    } else if let Some(project) = filter.project {
        request.project(project);
    }
    print_object_list(request.send()?, format)
}

fn get(
    api: lrzcc::Api,
    format: Format,
    id: &u32,
) -> Result<(), Box<dyn Error>> {
    print_single_object(api.user.get(*id)?, format)
}
