use crate::common::{
    ask_for_confirmation, print_object_list, print_single_object, Execute,
    Format,
};
use clap::{Args, Subcommand};
use std::error::Error;

#[cfg(not(feature = "user"))]
use crate::common::{find_id as user_find_id, find_id as project_find_id};
#[cfg(feature = "user")]
use crate::user::{
    project::find_id as project_find_id, user::find_id as user_find_id,
};

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct UserBudgetListFilter {
    #[clap(
        short,
        long,
        help = "Display user budgets of user with given name, ID, or OpenStack ID"
    )]
    user: Option<String>,

    #[clap(
        short,
        long,
        help = "Display user budgets of project with given name, ID, or OpenStack ID"
    )]
    project: Option<String>,

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
        #[clap(help = "Name, ID or OpenStack ID of the user of the budget")]
        user: String,

        #[clap(
            long,
            short,
            help = "Year of the budget, default: current year"
        )]
        year: Option<u32>,

        #[clap(long, short, help = "Amount of the budget, default: 0")]
        amount: Option<i64>,
    },

    #[clap(about = "Modify a user budget")]
    Modify {
        #[clap(help = "ID of the user budget")]
        id: u32,

        #[clap(long, short, help = "Amount of the budget")]
        amount: Option<u32>,

        #[clap(long, short, help = "Force the amount to be set", action)]
        force: bool,
    },

    #[clap(about = "Delete user budget with given ID")]
    Delete { id: u32 },
}
pub(crate) use UserBudgetCommand::*;

impl Execute for UserBudgetCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            List { filter } => list(api, format, filter),
            Get { id } => get(api, format, id),
            Create { user, year, amount } => {
                create(api, format, &user, *year, *amount)
            }
            Modify { id, amount, force } => {
                modify(api, format, *id, *amount, *force)
            }
            Delete { id } => delete(api, id),
        }
    }
}

fn list(
    api: lrzcc::Api,
    format: Format,
    filter: &UserBudgetListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.user_budget.list();
    if let Some(user) = &filter.user {
        let user_id = user_find_id(&api, &user)?;
        request.user(user_id);
    } else if let Some(project) = &filter.project {
        let project_id = project_find_id(&api, &project)?;
        request.project(project_id);
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
    user: &str,
    year: Option<u32>,
    amount: Option<i64>,
) -> Result<(), Box<dyn Error>> {
    let user_id = user_find_id(&api, user)?;
    let mut request = api.user_budget.create(user_id);
    if let Some(year) = year {
        request.year(year);
    }
    if let Some(amount) = amount {
        request.amount(amount);
    }
    print_single_object(request.send()?, format)
}

#[allow(clippy::too_many_arguments)]
fn modify(
    api: lrzcc::Api,
    format: Format,
    id: u32,
    amount: Option<u32>,
    force: bool,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.user_budget.modify(id);
    if let Some(amount) = amount {
        request.amount(amount);
    }
    if force {
        request.force();
    }
    print_single_object(request.send()?, format)
}

fn delete(api: lrzcc::Api, id: &u32) -> Result<(), Box<dyn Error>> {
    ask_for_confirmation()?;
    Ok(api.user_budget.delete(*id)?)
}
