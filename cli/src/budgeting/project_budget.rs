use crate::common::{
    ask_for_confirmation, print_object_list, print_single_object, Execute,
    Format,
};
use chrono::{DateTime, FixedOffset};
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
pub(crate) struct ProjectBudgetListFilter {
    #[clap(
        short,
        long,
        help = "Display project budgets of user with given name, ID, or OpenStack ID"
    )]
    user: Option<String>,

    #[clap(
        short,
        long,
        help = "Display project budgets of project with given name, ID, OpenStack ID"
    )]
    project: Option<String>,

    #[clap(short, long, help = "Display all project budgets", action)]
    all: bool,

    #[clap(short, long, help = "Display project budgets of the given year")]
    // TODO validate that this is a valid year
    year: Option<u32>,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct ProjectBudgetOverFilter {
    #[clap(short, long, help = "Filter project budget with given ID")]
    budget: Option<u32>,

    #[clap(
        short,
        long,
        help = "Filter for project budget of project with given name, ID, or OpenStack ID"
    )]
    project: Option<String>,

    #[clap(
        short,
        long,
        help = "Get information for all project budgets",
        action
    )]
    all: bool,
}

#[derive(Subcommand, Debug)]
pub(crate) enum ProjectBudgetCommand {
    #[clap(about = "List project budgets")]
    List {
        #[clap(flatten)]
        filter: ProjectBudgetListFilter,
    },

    #[clap(
        visible_alias = "show",
        about = "Show project budget with given ID"
    )]
    Get { id: u32 },

    #[clap(about = "Create a new project budget")]
    Create {
        #[clap(
            help = "Name, ID, or OpenStack ID of the project of the budget"
        )]
        project: String,

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

    #[clap(about = "List over status of project budgets")]
    Over {
        #[clap(flatten)]
        filter: ProjectBudgetOverFilter,

        #[clap(
            short,
            long,
            help = "Calculate over status up to this time [default: current time]"
        )]
        end: Option<DateTime<FixedOffset>>,

        #[clap(
            short,
            long,
            help = "Show detailed information about over status",
            action
        )]
        detail: bool,
    },
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
            } => create(api, format, project, *year, *amount),
            Modify { id, amount, force } => {
                modify(api, format, *id, *amount, *force)
            }
            Delete { id } => delete(api, id),
            Over {
                filter,
                end,
                detail,
            } => over(api, format, filter, *end, *detail),
        }
    }
}

fn list(
    api: lrzcc::Api,
    format: Format,
    filter: &ProjectBudgetListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.project_budget.list();
    if let Some(user) = &filter.user {
        let user_id = user_find_id(&api, user)?;
        request.user(user_id);
    } else if let Some(project) = &filter.project {
        let project_id = project_find_id(&api, project)?;
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
    print_single_object(api.project_budget.get(*id)?, format)
}

fn create(
    api: lrzcc::Api,
    format: Format,
    project: &str,
    year: Option<u32>,
    amount: Option<i64>,
) -> Result<(), Box<dyn Error>> {
    let project_id = project_find_id(&api, project)?;
    let mut request = api.project_budget.create(project_id);
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

fn over(
    api: lrzcc::Api,
    format: Format,
    filter: &ProjectBudgetOverFilter,
    end: Option<DateTime<FixedOffset>>,
    detail: bool,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.project_budget.over();
    if let Some(budget) = filter.budget {
        request.budget(budget);
    } else if let Some(project) = &filter.project {
        let project_id = project_find_id(&api, project)?;
        request.project(project_id);
    } else if filter.all {
        request.all();
    }
    if let Some(end) = end {
        request.end(end);
    }
    if detail {
        print_object_list(request.detail()?, format)
    } else {
        print_object_list(request.normal()?, format)
    }
}
