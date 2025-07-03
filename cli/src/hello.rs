use std::error::Error;

use clap::Subcommand;

use crate::common::{Execute, Format, print_single_object};

#[derive(Subcommand, Debug)]
pub(crate) enum HelloCommand {
    #[clap(about = "Hello admin command")]
    Admin,

    #[clap(about = "Hello user command")]
    User,
}
pub(crate) use HelloCommand::*;

impl Execute for HelloCommand {
    async fn execute(
        &self,
        api: avina::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            Admin => admin(api, format).await,
            User => user(api, format).await,
        }
    }
}

async fn admin(api: avina::Api, format: Format) -> Result<(), Box<dyn Error>> {
    print_single_object(api.hello.admin().await?, format)
}

async fn user(api: avina::Api, format: Format) -> Result<(), Box<dyn Error>> {
    print_single_object(api.hello.user().await?, format)
}
