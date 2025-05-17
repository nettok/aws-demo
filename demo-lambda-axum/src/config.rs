use crate::error::AppError;
use config::Config;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {}

pub fn load_app_config() -> Result<AppConfig, AppError> {
    let profile = std::env::var("CONFIG_PROFILE").unwrap_or_else(|_| "dev".to_owned());

    let conf = Config::builder()
        .add_source(config::File::with_name("config/default"))
        .add_source(config::File::with_name(&format!("config/{}", profile)).required(false))
        .add_source(config::Environment::default())
        .build()?;

    conf.try_deserialize::<AppConfig>()
        .map_err(|e| AppError::from(e))
}
