use std::error::Error;

pub(crate) fn usage(api: lrzcc::Api) -> Result<(), Box<dyn Error>> {
    println!("{}", serde_json::to_string(&api.usage.get()?)?);
    Ok(())
}
