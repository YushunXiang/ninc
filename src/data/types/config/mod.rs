mod basic;
mod esrep;

use anyhow::Result;
use serde::{Serialize, Deserialize};
pub use basic::*;
pub use esrep::*;

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub basic: Option<BasicConfig>,
  pub esrep: Option<EsrepConfig>,
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
      basic: None,
      esrep: None
    }
  }
}
