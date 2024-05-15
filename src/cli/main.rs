use clap::{Parser, Subcommand};
use colored::Colorize;
use lrzcc::Api;

mod common;
mod hello;

use crate::common::Execute;

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
        help = "LRZ CC API base URL",
        env = "LRZ_CC_API_URL",
        default_value = "https://cc.lrz.de:1337/api"
    )]
    url: String,

    // #[clap(flatten)]
    // credentials: CredentialArgs,
    #[clap(short, long, help = "Openstack API token", env = "OS_TOKEN")]
    token: String,

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

fn main() {
    let cli = Cli::parse();
    let api = Api::new(cli.url, cli.token);
    match match cli.command {
        Command::Hello { ref command } => command.execute(api),
    } {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}: {}", "error".bold().red(), e);
            std::process::exit(1);
        }
    }
}
