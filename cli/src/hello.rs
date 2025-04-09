use std::error::Error;

use clap::Subcommand;

use crate::common::{print_single_object, Execute, Format};

#[derive(Subcommand, Debug)]
pub(crate) enum HelloCommand {
    #[clap(about = "Hello admin command")]
    Admin,

    #[clap(about = "Hello user command")]
    User,
}
pub(crate) use HelloCommand::*;

impl Execute for HelloCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            Admin => admin(api, format),
            User => user(api, format),
        }
    }
}

fn admin(api: lrzcc::Api, format: Format) -> Result<(), Box<dyn Error>> {
    print_single_object(api.hello.admin()?, format)
}

fn user(api: lrzcc::Api, format: Format) -> Result<(), Box<dyn Error>> {
    print_single_object(api.hello.user()?, format)
}
