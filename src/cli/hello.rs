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
            HelloCommand::Admin {} => admin(api),
            HelloCommand::User {} => user(api),
        }
    }
}

fn admin(api: lrzcc::Api) -> Result<(), Box<dyn Error>> {
    api.hello.admin();
    Ok(())
}

fn user(api: lrzcc::Api) -> Result<(), Box<dyn Error>> {
    api.hello.user();
    Ok(())
}
