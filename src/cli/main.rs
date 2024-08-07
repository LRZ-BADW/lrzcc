use chrono::{DateTime, FixedOffset};
use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use common::current_year;
use lrzcc::Api;
use std::process::ExitCode;
use std::str::FromStr;

mod common;
mod token;

#[cfg(feature = "accounting")]
mod accounting;
#[cfg(feature = "budgeting")]
mod budgeting;
#[cfg(feature = "hello")]
mod hello;
#[cfg(feature = "pricing")]
mod pricing;
#[cfg(feature = "quota")]
mod quota;
#[cfg(feature = "resources")]
mod resources;
#[cfg(feature = "user")]
mod user;

use common::{Execute, Format, TableFormat};
use token::Token;

#[derive(Args, Debug)]
#[group(required = true, multiple = true)]
struct CredentialArgs {
    #[clap(short, long, help = "Openstack API token", env = "OS_TOKEN")]
    token: Option<String>,

    #[clap(
        short = 'A',
        long,
        help = "Openstack authentication URL",
        env = "OS_AUTH_URL",
        requires = "username",
        requires = "password",
        requires = "project_name",
        requires = "user_domain_name",
        requires = "project_domain_id"
    )]
    auth_url: Option<String>,

    #[clap(
        short = 'U',
        long,
        help = "Openstack username",
        env = "OS_USERNAME",
        requires = "auth_url",
        requires = "password",
        requires = "project_name",
        requires = "user_domain_name",
        value_enum,
        requires = "project_domain_id"
    )]
    username: Option<String>,

    #[clap(
        short = 'P',
        long,
        help = "Openstack password",
        env = "OS_PASSWORD",
        requires = "auth_url",
        requires = "username",
        requires = "project_name",
        requires = "user_domain_name",
        requires = "project_domain_id"
    )]
    password: Option<String>,

    #[clap(
        short = 'N',
        long,
        help = "Openstack project name",
        env = "OS_PROJECT_NAME",
        requires = "auth_url",
        requires = "username",
        requires = "password",
        requires = "user_domain_name",
        requires = "project_domain_id"
    )]
    project_name: Option<String>,

    #[clap(
        short = 'D',
        long,
        help = "Openstack user domain name",
        env = "OS_USER_DOMAIN_NAME",
        requires = "auth_url",
        requires = "username",
        requires = "password",
        requires = "project_name",
        requires = "project_domain_id"
    )]
    user_domain_name: Option<String>,

    #[clap(
        short = 'I',
        long,
        help = "Openstack project domain ID",
        env = "OS_PROJECT_DOMAIN_ID",
        requires = "auth_url",
        requires = "username",
        requires = "password",
        requires = "project_name",
        requires = "user_domain_name"
    )]
    project_domain_id: Option<String>,
}

#[derive(Parser, Debug)]
#[command(name = "lrzcc")]
#[command(author = "Sandro-Alessio Gierens <sandro@gierens.de>")]
#[command(version = "0.1.0")]
#[command(
    about = "CLI client for the LRZ-specific features of the Openstack-based LRZ Compute Cloud."
)]
struct Cli {
    #[clap(
        short,
        long,
        help = "Custom LRZ Compute Cloud API base URL",
        env = "LRZ_CC_API_URL",
        default_value = "https://cc.lrz.de:1337/api"
    )]
    url: String,

    #[clap(flatten)]
    credentials: CredentialArgs,

    // TODO allow specifying user by name as well
    #[clap(short, long, help = "ID of the user to impersonate")]
    impersonate: Option<u32>,

    #[clap(
        value_enum,
        short,
        long,
        help = "Format of the output",
        default_value_t = Format::Table(TableFormat::Rounded)
    )]
    format: Format,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[cfg(feature = "resources")]
    #[clap(about = "Flavor command")]
    Flavor {
        #[clap(subcommand)]
        command: resources::FlavorCommand,
    },

    #[cfg(feature = "resources")]
    #[clap(about = "Flavor group command")]
    FlavorGroup {
        #[clap(subcommand)]
        command: resources::FlavorGroupCommand,
    },

    #[cfg(feature = "resources")]
    #[clap(about = "Usage command")]
    Usage,

    #[cfg(feature = "pricing")]
    #[clap(about = "Flavor price command")]
    FlavorPrice {
        #[clap(subcommand)]
        command: pricing::FlavorPriceCommand,
    },

    #[cfg(feature = "quota")]
    #[clap(about = "Flavor quota command")]
    FlavorQuota {
        #[clap(subcommand)]
        command: quota::FlavorQuotaCommand,
    },

    #[cfg(feature = "hello")]
    #[clap(about = "Hello command")]
    Hello {
        #[clap(subcommand)]
        command: hello::HelloCommand,
    },

    #[cfg(feature = "user")]
    #[clap(about = "User command")]
    Project {
        #[clap(subcommand)]
        command: user::ProjectCommand,
    },

    #[cfg(feature = "user")]
    #[clap(about = "User command")]
    User {
        #[clap(subcommand)]
        command: user::UserCommand,
    },

    #[cfg(feature = "accounting")]
    #[clap(about = "Server state command")]
    ServerState {
        #[clap(subcommand)]
        command: accounting::ServerStateCommand,
    },

    #[cfg(feature = "accounting")]
    #[clap(about = "Server cost command")]
    ServerCost {
        #[clap(
            long,
            short,
            help = "Begin of the period to calculate the cost for (default: beginning of the running year)"
        )]
        begin: Option<DateTime<FixedOffset>>,

        #[clap(
            long,
            short,
            help = "End of the period to calculate the cost for (default: now)"
        )]
        end: Option<DateTime<FixedOffset>>,
    },

    #[cfg(feature = "budgeting")]
    #[clap(about = "Project budget command")]
    ProjectBudget {
        #[clap(subcommand)]
        command: budgeting::ProjectBudgetCommand,
    },

    #[cfg(feature = "budgeting")]
    #[clap(about = "User budget command")]
    UserBudget {
        #[clap(subcommand)]
        command: budgeting::UserBudgetCommand,
    },

    #[cfg(feature = "budgeting")]
    #[clap(about = "Budget over tree command")]
    BudgetOverTree {
        #[clap(flatten)]
        filter: budgeting::BudgetOverTreeFilter,

        #[clap(
            short,
            long,
            help = "End up to which to calculate the budget over tree (default: current time)"
        )]
        end: Option<DateTime<FixedOffset>>,
    },

    #[cfg(feature = "budgeting")]
    #[clap(about = "Budget bulk create command")]
    BudgetBulkCreate {
        #[clap(
            short,
            long,
            default_value_t = current_year(),
            help = "Year for which to bulk create budgets (default: current year)"
        )]
        year: i32,
    },
}

// TODO: missing commands:
// - user-budget over
// - project-budget over
// - flavor usage
// - flavor-group usage
// - server-consumption
// In progress:
// - server-cost

fn main() -> ExitCode {
    let cli = Cli::parse();
    let token = match cli.credentials.token {
        Some(token) => Token::from_str(token.as_str()).unwrap(),
        None => {
            let auth_url = cli.credentials.auth_url.unwrap();
            let username = cli.credentials.username.unwrap();
            let password = cli.credentials.password.unwrap();
            let project_name = cli.credentials.project_name.unwrap();
            let user_domain_name = cli.credentials.user_domain_name.unwrap();
            let project_domain_id = cli.credentials.project_domain_id.unwrap();
            match Token::new(
                auth_url.as_str(),
                username.as_str(),
                password.as_str(),
                project_name.as_str(),
                user_domain_name.as_str(),
                project_domain_id.as_str(),
            ) {
                Ok(token) => token,
                Err(error) => {
                    eprintln!("{}: {}", "error".bold().red(), error);
                    std::process::exit(1);
                }
            }
        }
    };
    let api =
        match Api::new(cli.url, token.as_ref().to_string(), cli.impersonate) {
            Ok(api) => api,
            Err(error) => {
                eprintln!("{}: {}", "error".bold().red(), error);
                return ExitCode::FAILURE;
            }
        };
    #[cfg(any(
        feature = "accounting",
        feature = "budgeting",
        feature = "hello",
        feature = "pricing",
        feature = "resources",
        feature = "user",
    ))]
    match match cli.command {
        #[cfg(feature = "hello")]
        Command::Hello { ref command } => command.execute(api, cli.format),
        #[cfg(feature = "user")]
        Command::User { ref command } => command.execute(api, cli.format),
        #[cfg(feature = "user")]
        Command::Project { ref command } => command.execute(api, cli.format),
        #[cfg(feature = "pricing")]
        Command::FlavorPrice { ref command } => {
            command.execute(api, cli.format)
        }
        #[cfg(feature = "quota")]
        Command::FlavorQuota { ref command } => {
            command.execute(api, cli.format)
        }
        #[cfg(feature = "resources")]
        Command::Flavor { ref command } => command.execute(api, cli.format),
        #[cfg(feature = "resources")]
        Command::FlavorGroup { ref command } => {
            command.execute(api, cli.format)
        }
        #[cfg(feature = "resources")]
        Command::Usage => resources::usage(api),
        #[cfg(feature = "accounting")]
        Command::ServerState { ref command } => {
            command.execute(api, cli.format)
        }
        #[cfg(feature = "accounting")]
        Command::ServerCost { begin, end } => {
            accounting::server_cost(api, cli.format, begin, end)
        }
        #[cfg(feature = "budgeting")]
        Command::ProjectBudget { ref command } => {
            command.execute(api, cli.format)
        }
        #[cfg(feature = "budgeting")]
        Command::UserBudget { ref command } => command.execute(api, cli.format),
        #[cfg(feature = "budgeting")]
        Command::BudgetOverTree { filter, end } => {
            budgeting::budget_over_tree(api, filter, end)
        }
        #[cfg(feature = "budgeting")]
        Command::BudgetBulkCreate { year } => {
            budgeting::budget_bulk_create(api, cli.format, year)
        }
    } {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}: {}", "error".bold().red(), error);
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
