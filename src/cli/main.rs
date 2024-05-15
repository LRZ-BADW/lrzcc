use clap::{Args, Parser, Subcommand};
use lrzcc::hello_world;

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
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(about = "Hello command")]
    Hello,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Hello => hello_world(),
    };
}
