//! **ATTENTION:** This has been renamed to [**avina-api**](https://crates.io/crates/avina-api).
use lrzcc_api::configuration::get_configuration;
use lrzcc_api::startup::Application;
use lrzcc_api::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber =
        get_subscriber("lrzcc-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration =
        get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
