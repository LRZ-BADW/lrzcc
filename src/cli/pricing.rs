use crate::common::{print_object_list, print_single_object, Execute, Format};
use chrono::{DateTime, Utc};
use clap::Subcommand;
use std::error::Error;

#[derive(Subcommand, Debug)]
pub(crate) enum FlavorPriceCommand {
    #[clap(about = "List users")]
    List,

    #[clap(about = "Show flavor price with given ID")]
    Get { id: u32 },

    #[clap(about = "Create a new flavor price")]
    Create {
        #[clap(help = "ID of the flavor of the price")]
        flavor: u32,

        #[clap(help = "User class of the price (1-6)")]
        user_class: u32,

        #[clap(long, short, help = "Unit price of the flavor, default: 0.0")]
        price: Option<f64>,

        #[clap(long, short, help = "Start time of the price, default: now")]
        start_time: Option<DateTime<Utc>>,
    },

    #[clap(about = "Delete flavor price with given ID")]
    Delete { id: u32 },
}
pub(crate) use FlavorPriceCommand::*;

impl Execute for FlavorPriceCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            List => list(api, format),
            Get { id } => get(api, format, id),
            Create {
                flavor,
                user_class,
                price,
                start_time,
            } => create(api, format, *flavor, *user_class, *price, *start_time),
            Delete { id } => delete(api, id),
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

fn create(
    api: lrzcc::Api,
    format: Format,
    flavor: u32,
    user_class: u32,
    price: Option<f64>,
    start_time: Option<DateTime<Utc>>,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.flavor_price.create(flavor, user_class);
    if let Some(price) = price {
        request.price(price);
    }
    if let Some(start_time) = start_time {
        request.start_time(start_time);
    }
    print_single_object(request.send()?, format)
}

fn delete(api: lrzcc::Api, id: &u32) -> Result<(), Box<dyn Error>> {
    // TODO dangerous operations like this one should be protected by a
    // confirmation prompt
    Ok(api.flavor_price.delete(*id)?)
}
