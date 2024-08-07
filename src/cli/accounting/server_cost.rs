use crate::common::{print_single_object, Format};
use chrono::{DateTime, FixedOffset};
use std::error::Error;

pub(crate) fn server_cost(
    api: lrzcc::Api,
    format: Format,
    begin: Option<DateTime<FixedOffset>>,
    end: Option<DateTime<FixedOffset>>,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.server_cost.get();
    if let Some(begin) = begin {
        request.begin(begin);
    }
    if let Some(end) = end {
        request.end(end);
    }
    print_single_object(request.send()?, format)
}
