use avina_api::{
    configuration::get_configuration,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber =
        get_subscriber("avina-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration =
        get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
