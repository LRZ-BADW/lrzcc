use crate::common::{print_object_list, Execute, Format};
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct ProjectBudgetListFilter {
    #[clap(
        short,
        long,
        help = "Display project budgets of user with given ID"
    )]
    // TODO validate that this is a valid user ID
    user: Option<u32>,

    #[clap(
        short,
        long,
        help = "Display project budgets of project with given ID"
    )]
    // TODO validate that this is a valid project ID
    project: Option<u32>,

    #[clap(short, long, help = "Display all project budgets", action)]
    all: bool,

    #[clap(short, long, help = "Display project budgets of the given year")]
    // TODO validate that this is a valid year
    year: Option<u32>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum ProjectBudgetCommand {
    #[clap(about = "List project budgets")]
    List {
        #[clap(flatten)]
        filter: ProjectBudgetListFilter,
    },
}

impl Execute for ProjectBudgetCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            ProjectBudgetCommand::List { filter } => list(api, format, filter),
        }
    }
}

fn list(
    api: lrzcc::Api,
    format: Format,
    filter: &ProjectBudgetListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.project_budget.list();
    if let Some(user) = filter.user {
        request.user(user);
    } else if let Some(project) = filter.project {
        request.project(project);
    } else if filter.all {
        request.all();
    }
    if let Some(year) = filter.year {
        request.year(year);
    }
    print_object_list(request.send()?, format)
}
