use crate::common::{print_single_object, Format};
use std::error::Error;

pub(crate) fn budget_bulk_create(
    api: lrzcc::Api,
    format: Format,
    year: i32,
) -> Result<(), Box<dyn Error>> {
    print_single_object(&api.budget_bulk_create.run(year)?, format)
}
