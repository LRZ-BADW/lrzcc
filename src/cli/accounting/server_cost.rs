use crate::common::{print_single_object, Format};
use std::error::Error;

pub(crate) fn server_cost(
    api: lrzcc::Api,
    format: Format,
) -> Result<(), Box<dyn Error>> {
    print_single_object(&api.server_cost.get()?, format)
}
