use crate::common::{print_object_list, Execute, Format};
use clap::Subcommand;
use std::error::Error;

#[derive(Subcommand, Debug)]
pub(crate) enum FlavorPriceCommand {
    #[clap(about = "List users")]
    List,
}

impl Execute for FlavorPriceCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            FlavorPriceCommand::List => list(api, format),
        }
    }
}

fn list(api: lrzcc::Api, format: Format) -> Result<(), Box<dyn Error>> {
    let request = api.flavor_price.list();
    print_object_list(request.send()?, format)
}
