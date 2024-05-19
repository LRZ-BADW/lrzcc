use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use lrzcc::Api;

mod common;
mod hello;

use crate::common::Execute;

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

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(about = "Hello command")]
    Hello {
        #[clap(subcommand)]
        command: hello::HelloCommand,
    },
}

fn issue_api_token(
    _auth_url: &str,
    _username: &str,
    _password: &str,
    _project_name: &str,
    _user_domain_name: &str,
    _project_domain_id: &str,
) -> Result<String, anyhow::Error> {
    todo!()
}

fn main() {
    let cli = Cli::parse();
    let token = match cli.credentials.token {
        Some(token) => token,
        None => {
            let auth_url = cli.credentials.auth_url.unwrap();
            let username = cli.credentials.username.unwrap();
            let password = cli.credentials.password.unwrap();
            let project_name = cli.credentials.project_name.unwrap();
            let user_domain_name = cli.credentials.user_domain_name.unwrap();
            let project_domain_id = cli.credentials.project_domain_id.unwrap();
            match issue_api_token(
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
    let api = Api::new(cli.url, token);
    match match cli.command {
        Command::Hello { ref command } => command.execute(api),
    } {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}: {}", "error".bold().red(), error);
            std::process::exit(1);
        }
    }
}
