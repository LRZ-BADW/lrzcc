use crate::common::{print_object_list, print_single_object, Execute, Format};
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct ProjectListFilter {
    #[clap(short, long, help = "Display all projects", action)]
    all: bool,

    #[clap(short, long, help = "Display projects of given user class")]
    // TODO: use enum for this
    user_class: Option<u32>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum ProjectCommand {
    #[clap(about = "List projects")]
    List {
        #[clap(flatten)]
        filter: ProjectListFilter,
    },

    #[clap(about = "Show project with given ID")]
    Get { id: u32 },
}

impl Execute for ProjectCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            ProjectCommand::List { filter } => list(api, format, filter),
            ProjectCommand::Get { id } => get(api, format, id),
        }
    }
}

fn list(
    api: lrzcc::Api,
    format: Format,
    filter: &ProjectListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.project.list();
    if filter.all {
        request.all();
    } else if let Some(user_class) = filter.user_class {
        request.user_class(user_class);
    }
    print_object_list(request.send()?, format)
}

fn get(
    api: lrzcc::Api,
    format: Format,
    id: &u32,
) -> Result<(), Box<dyn Error>> {
    print_single_object(api.project.get(*id)?, format)
}
