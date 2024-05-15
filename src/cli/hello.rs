use crate::common::Execute;
use clap::Subcommand;
use std::error::Error;

#[derive(Subcommand, Debug)]
pub(crate) enum HelloCommand {
    #[clap(about = "Hello admin command")]
    Admin,

    #[clap(about = "Hello user command")]
    User,
}

impl Execute for HelloCommand {
    fn execute(&self, api: lrzcc::Api) -> Result<(), Box<dyn Error>> {
        match self {
            HelloCommand::Admin {} => hello_admin(api),
            HelloCommand::User {} => hello_user(api),
        }
    }
}

fn hello_admin(api: lrzcc::Api) -> Result<(), Box<dyn Error>> {
    api.hello_admin();
    Ok(())
}

fn hello_user(api: lrzcc::Api) -> Result<(), Box<dyn Error>> {
    api.hello_user();
    Ok(())
}
