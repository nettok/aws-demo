use dotenvy::dotenv;
use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use serde::Deserialize;
use util::config::load_app_config;
use util::tracing;

refinery::embed_migrations!("migrations");

#[derive(Clone, Deserialize)]
struct AppConfig {
    ca_certs: String,
    postgres: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    tracing::init_tracing_default_subscriber();

    let app_config = load_app_config::<AppConfig>()?;
    let shared_config = &app_config;

    run(service_fn(move |event: LambdaEvent<String>| async move {
        handler(shared_config, event).await
    }))
    .await?;
    Ok(())
}

async fn handler(config: &AppConfig, event: LambdaEvent<String>) -> Result<(), Error> {
    if event.payload == "migrate" {
        migrate(config).await?;
    }

    Ok(())
}

async fn migrate(config: &AppConfig) -> Result<(), Error> {
    use native_tls::{Certificate, TlsConnector};
    use postgres_native_tls::MakeTlsConnector;
    use std::fs;

    let cert = fs::read(&config.ca_certs)?;
    let cert = Certificate::from_pem(&cert)?;
    let connector = TlsConnector::builder().add_root_certificate(cert).build()?;

    let connector = MakeTlsConnector::new(connector);

    let (mut client, connection) = tokio_postgres::connect(&config.postgres, connector).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    migrations::runner().run_async(&mut client).await?;

    Ok(())
}
