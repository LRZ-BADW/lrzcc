use crate::common::{print_object_list, Execute, Format};
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct UserListFilter {
    #[clap(short, long, help = "Display all users", action)]
    all: bool,

    #[clap(short, long, help = "Display users of project with given ID")]
    project: Option<u32>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum UserCommand {
    #[clap(about = "List users")]
    List {
        #[clap(flatten)]
        filter: UserListFilter,
    },
}

impl Execute for UserCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            UserCommand::List { filter } => list(api, format, filter),
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
