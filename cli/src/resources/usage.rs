use std::error::Error;

pub(crate) fn usage(api: avina::Api) -> Result<(), Box<dyn Error>> {
    println!("{}", serde_json::to_string(&api.usage.get()?)?);
    Ok(())
}
