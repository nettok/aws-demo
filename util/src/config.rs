use config::Config;
use serde::Deserialize;
use std::str::FromStr;
use strum::{Display, EnumString};

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Clone, Display, EnumString)]
enum RunProfile {
    #[strum(serialize = "dev")]
    Dev,

    #[strum(serialize = "prod")]
    Prod,
}

pub fn load_app_config<'de, T: Clone + Deserialize<'de>>() -> Result<T, BoxError> {
    let default_run_profile = RunProfile::Dev;

    let profile = std::env::var("RUN_PROFILE")
        .map(|env_profile| {
            RunProfile::from_str(&env_profile).unwrap_or(default_run_profile.clone())
        })
        .unwrap_or(default_run_profile)
        .to_string();

    let conf = Config::builder()
        .add_source(config::File::with_name("config/default"))
        .add_source(config::File::with_name(&format!("config/{}", profile)).required(false))
        .add_source(config::Environment::default())
        .build()?;

    conf.try_deserialize::<T>().map_err(|e| e.into())
}
