use crate::common::{print_object_list, print_single_object, Execute, Format};
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

    #[clap(about = "Show user budget with given ID")]
    Get { id: u32 },

    #[clap(about = "Create a new user budget")]
    Create {
        #[clap(help = "Id of the user of the budget")]
        user: u32,

        #[clap(
            long,
            short,
            help = "Year of the budget, default: current year"
        )]
        year: Option<u32>,

        #[clap(long, short, help = "Amount of the budget, default: 0")]
        amount: Option<i64>,
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
            UserBudgetCommand::Get { id } => get(api, format, id),
            UserBudgetCommand::Create { user, year, amount } => {
                create(api, format, *user, *year, *amount)
            }
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

fn get(
    api: lrzcc::Api,
    format: Format,
    id: &u32,
) -> Result<(), Box<dyn Error>> {
    print_single_object(api.user_budget.get(*id)?, format)
}

fn create(
    api: lrzcc::Api,
    format: Format,
    user: u32,
    year: Option<u32>,
    amount: Option<i64>,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.user_budget.create(user);
    if let Some(year) = year {
        request.year(year);
    }
    if let Some(amount) = amount {
        request.amount(amount);
    }
    print_single_object(request.send()?, format)
}
