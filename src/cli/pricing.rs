use crate::common::{print_object_list, print_single_object, Execute, Format};
use clap::Subcommand;
use std::error::Error;

#[derive(Subcommand, Debug)]
pub(crate) enum FlavorPriceCommand {
    #[clap(about = "List users")]
    List,

    #[clap(about = "Show flavor price with given ID")]
    Get { id: u32 },
}

impl Execute for FlavorPriceCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            FlavorPriceCommand::List => list(api, format),
            FlavorPriceCommand::Get { id } => get(api, format, id),
        }
    }
}

fn list(api: lrzcc::Api, format: Format) -> Result<(), Box<dyn Error>> {
    let request = api.flavor_price.list();
    print_object_list(request.send()?, format)
}

fn get(
    api: lrzcc::Api,
    format: Format,
    id: &u32,
) -> Result<(), Box<dyn Error>> {
    print_single_object(api.flavor_price.get(*id)?, format)
}
