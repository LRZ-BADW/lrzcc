use crate::common::{Execute, Format};
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
    let hello = api.hello.admin()?;
    match format {
        Format::Json => println!("{}", serde_json::to_string(&hello)?),
        _ => println!("{}", hello),
    }
    Ok(())
}

fn user(api: lrzcc::Api, format: Format) -> Result<(), Box<dyn Error>> {
    let hello = api.hello.user()?;
    match format {
        Format::Json => println!("{}", serde_json::to_string(&hello)?),
        _ => println!("{}", hello),
    }
    Ok(())
}
