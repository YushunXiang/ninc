use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BasicConfig {
  pub username: String,
  pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub basic: BasicConfig,
}

impl Config {
  pub async fn load(path: &str) -> Result<Self> {
    let config_str = tokio::fs::read_to_string(path).await?;
    let config = toml::from_str(&config_str)?;
    Ok(config)
  }
  pub async fn save(&self, path: &str) -> Result<()> {
    let config_str = toml::to_string_pretty(&self)?;
    tokio::fs::write(path, config_str).await?;
    Ok(())
  }
  pub fn new() -> Self {
    Config {
      basic: BasicConfig {
        username: String::new(),
        password: String::new(),
      }
    }
  }
}
