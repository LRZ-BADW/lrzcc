use crate::common::{print_object_list, Execute, Format};
use clap::Subcommand;
use std::error::Error;

#[derive(Subcommand, Debug)]
pub(crate) enum UserCommand {
    #[clap(about = "List users")]
    List,
}

impl Execute for UserCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            UserCommand::List {} => list(api, format),
        }
    }
}

fn list(api: lrzcc::Api, format: Format) -> Result<(), Box<dyn Error>> {
    print_object_list(api.user.list()?, format)
}
