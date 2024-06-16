use crate::common::{print_object_list, Execute, Format};
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct UserBudgetListFilter {
    #[clap(short, long, help = "Display user budgets of user with given ID")]
    // TODO validate that this is a valid user ID
    user: Option<u32>,

    #[clap(
        short,
        long,
        help = "Display user budgets of project with given ID"
    )]
    // TODO validate that this is a valid project ID
    project: Option<u32>,

    #[clap(short, long, help = "Display all user budgets", action)]
    all: bool,

    #[clap(short, long, help = "Display user budgets of the given year")]
    // TODO validate that this is a valid year
    year: Option<u32>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum UserBudgetCommand {
    #[clap(about = "List user budgets")]
    List {
        #[clap(flatten)]
        filter: UserBudgetListFilter,
    },
}

impl Execute for UserBudgetCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            UserBudgetCommand::List { filter } => list(api, format, filter),
        }
    }
}

fn list(
    api: lrzcc::Api,
    format: Format,
    filter: &UserBudgetListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.user_budget.list();
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
