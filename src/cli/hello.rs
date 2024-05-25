use crate::common::{print_single_object, Execute, Format};
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
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            HelloCommand::Admin {} => admin(api, format),
            HelloCommand::User {} => user(api, format),
        }
    }
}

fn admin(api: lrzcc::Api, format: Format) -> Result<(), Box<dyn Error>> {
    print_single_object(api.hello.admin()?, format)
}

fn user(api: lrzcc::Api, format: Format) -> Result<(), Box<dyn Error>> {
    print_single_object(api.hello.user()?, format)
}
