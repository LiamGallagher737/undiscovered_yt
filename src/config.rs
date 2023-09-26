use crate::BINARY_NAME;
use anyhow::{anyhow, Result};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub title_text: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: "".to_string(),
            title_text: true,
        }
    }
}

impl Config {
    pub fn load() -> Result<Config> {
        let path = config_path()?;
        if !path.exists() {
            fs::create_dir_all(
                path.parent()
                    .ok_or_else(|| anyhow!("Unable to get directory from config path"))?,
            )?;
            Self::save(&Config::default())?;
            return Ok(Config::default());
        }
        let config_text = fs::read_to_string(path)?;
        Ok(toml::from_str(&config_text)?)
    }

    pub fn save(&self) -> Result<()> {
        let config_text = toml::to_string_pretty(self)?;
        fs::write(config_path()?, config_text)?;
        Ok(())
    }
}

fn config_path() -> Result<PathBuf> {
    let mut path = BaseDirs::new()
        .ok_or_else(|| anyhow!("Failed to get config path"))?
        .config_dir()
        .to_path_buf();
    path.push(BINARY_NAME);
    path.push("config.toml");
    Ok(path)
}
