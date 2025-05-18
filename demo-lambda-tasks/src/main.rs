use util::config::load_app_config;
use util::tracing;
use dotenvy::dotenv;
use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
struct AppConfig {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    tracing::init_tracing_default_subscriber();

    let app_config = load_app_config::<AppConfig>()?;

    run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<String>) -> Result<String, Error> {
    Ok(event.payload)
}
