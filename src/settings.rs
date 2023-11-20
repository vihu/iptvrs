use anyhow::Result;
use config::{Config, Environment, File};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub url: String,
    pub db_path: PathBuf,
    pub player_path: PathBuf,
}

impl Settings {
    pub fn new<P: AsRef<Path>>(path: Option<P>) -> Result<Self, config::ConfigError> {
        let mut builder = Config::builder();

        match path {
            Some(file) => {
                builder = builder
                    .add_source(File::with_name(&file.as_ref().to_string_lossy()).required(false));
            }
            None => {
                if let Some(mut path) = config_dir() {
                    path.push("iptvrs");
                    path.push("settings.toml");
                    builder = builder.add_source(File::from(path).required(false));
                }
            }
        }

        builder
            .add_source(Environment::with_prefix("iptvrs").separator("_"))
            .build()
            .and_then(|config| config.try_deserialize())
    }
}
