use std::error::Error;

use crate::common::{Format, print_single_object};

pub(crate) async fn budget_bulk_create(
    api: avina::Api,
    format: Format,
    year: i32,
) -> Result<(), Box<dyn Error>> {
    print_single_object(&api.budget_bulk_create.run(year).await?, format)
}
