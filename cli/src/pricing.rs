use crate::common::{
    ask_for_confirmation, print_object_list, print_single_object, Execute,
    Format,
};
use chrono::{DateTime, FixedOffset};
use clap::Subcommand;
use std::error::Error;

#[cfg(not(feature = "resources"))]
use crate::common::find_id as flavor_find_id;
#[cfg(feature = "resources")]
use crate::resources::flavor::find_id as flavor_find_id;

#[derive(Subcommand, Debug)]
pub(crate) enum FlavorPriceCommand {
    #[clap(about = "List users")]
    List,

    #[clap(visible_alias = "show", about = "Show flavor price with given ID")]
    Get { id: u32 },

    #[clap(about = "Create a new flavor price")]
    Create {
        #[clap(
            help = "Name, ID, or OpenStack UUIDv4 of the flavor of the price"
        )]
        flavor: String,

        #[clap(help = "User class of the price (1-6)")]
        user_class: u32,

        #[clap(long, short, help = "Unit price of the flavor, default: 0.0")]
        price: Option<f64>,

        #[clap(long, short, help = "Start time of the price, default: now")]
        start_time: Option<DateTime<FixedOffset>>,
    },

    #[clap(about = "Modify a flavor price")]
    Modify {
        #[clap(help = "ID of the flavor price")]
        id: u32,

        #[clap(
            long,
            short,
            help = "Name, ID, or OpenStack UUIDv4 Flavor the price belongs to"
        )]
        flavor: Option<String>,

        #[clap(long, short, help = "User class of the price (1-6)")]
        user_class: Option<u32>,

        #[clap(long, short, help = "Unit price of the flavor")]
        price: Option<f64>,

        #[clap(long, short, help = "Start time of the flavor price")]
        start_time: Option<DateTime<FixedOffset>>,
    },

    #[clap(about = "Delete flavor price with given ID")]
    Delete { id: u32 },

    #[clap(about = "Initialize first flavor prices")]
    Initialize,
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
            } => create(api, format, flavor, *user_class, *price, *start_time),
            Modify {
                id,
                flavor,
                user_class,
                price,
                start_time,
            } => modify(
                api,
                format,
                *id,
                flavor.to_owned(),
                *user_class,
                *price,
                *start_time,
            ),
            Delete { id } => delete(api, id),
            Initialize => initialize(api, format),
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
    flavor: &str,
    user_class: u32,
    price: Option<f64>,
    start_time: Option<DateTime<FixedOffset>>,
) -> Result<(), Box<dyn Error>> {
    let flavor_id = flavor_find_id(&api, flavor)?;
    let mut request = api.flavor_price.create(flavor_id, user_class);
    if let Some(price) = price {
        request.price(price);
    }
    if let Some(start_time) = start_time {
        request.start_time(start_time);
    }
    print_single_object(request.send()?, format)
}

fn modify(
    api: lrzcc::Api,
    format: Format,
    id: u32,
    flavor: Option<String>,
    user_class: Option<u32>,
    unit_price: Option<f64>,
    start_time: Option<DateTime<FixedOffset>>,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.flavor_price.modify(id);
    if let Some(flavor) = flavor {
        let flavor_id = flavor_find_id(&api, &flavor)?;
        request.flavor(flavor_id);
    }
    if let Some(user_class) = user_class {
        request.user_class(user_class);
    }
    if let Some(unit_price) = unit_price {
        request.unit_price(unit_price);
    }
    if let Some(start_time) = start_time {
        request.start_time(start_time);
    }
    print_single_object(request.send()?, format)
}

fn delete(api: lrzcc::Api, id: &u32) -> Result<(), Box<dyn Error>> {
    ask_for_confirmation()?;
    Ok(api.flavor_price.delete(*id)?)
}

fn initialize(api: lrzcc::Api, format: Format) -> Result<(), Box<dyn Error>> {
    let result = api.flavor_price.initialize()?;
    print_single_object(result, format)
}
