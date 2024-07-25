use crate::common::{
    ask_for_confirmation, print_object_list, print_single_object, Execute,
    Format,
};
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
    // TODO use find_id
    user: Option<u32>,

    #[clap(
        short,
        long,
        help = "Display project budgets of project with given ID"
    )]
    // TODO validate that this is a valid project ID
    // TODO use find_id
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

    #[clap(about = "Show project budget with given ID")]
    Get { id: u32 },

    #[clap(about = "Create a new project budget")]
    Create {
        #[clap(help = "Id of the project of the budget")]
        // TODO use find_id
        project: u32,

        #[clap(
            long,
            short,
            help = "Year of the budget, default: current year"
        )]
        year: Option<u32>,

        #[clap(long, short, help = "Amount of the budget, default: 0")]
        amount: Option<i64>,
    },

    #[clap(about = "Modify a project budget")]
    Modify {
        #[clap(help = "ID of the project budget")]
        id: u32,

        #[clap(long, short, help = "Amount of the budget")]
        amount: Option<u32>,

        #[clap(long, short, help = "Force the amount to be set", action)]
        force: bool,
    },

    #[clap(about = "Delete project budget with given ID")]
    Delete { id: u32 },
}
pub(crate) use ProjectBudgetCommand::*;

impl Execute for ProjectBudgetCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            List { filter } => list(api, format, filter),
            Get { id } => get(api, format, id),
            Create {
                project,
                year,
                amount,
            } => create(api, format, *project, *year, *amount),
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

fn get(
    api: lrzcc::Api,
    format: Format,
    id: &u32,
) -> Result<(), Box<dyn Error>> {
    print_single_object(api.project_budget.get(*id)?, format)
}

fn create(
    api: lrzcc::Api,
    format: Format,
    project: u32,
    year: Option<u32>,
    amount: Option<i64>,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.project_budget.create(project);
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
    let mut request = api.project_budget.modify(id);
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
    Ok(api.project_budget.delete(*id)?)
}
