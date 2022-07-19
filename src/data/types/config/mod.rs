mod basic;
mod esrep;

use anyhow::Result;
use serde::{Serialize, Deserialize};
use crate::tools::get_save_path;
pub use basic::*;
pub use esrep::*;

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub basic: Option<BasicConfig>,
  pub esrep: Option<EsrepConfig>,
}

impl Config {
  pub async fn load(path: &str) -> Result<Self> {
    let file = get_save_path(path);
    let config_str = tokio::fs::read_to_string(file).await?;
    let config = toml::from_str(&config_str)?;
    Ok(config)
  }
  pub async fn save(&self, path: &str) -> Result<()> {
    println!("Here");
    let config_str = toml::to_string_pretty(&self)?;
    println!("Here");
    let file = get_save_path(path);
    tokio::fs::write(file, config_str).await?;
    Ok(())
  }
  pub fn new() -> Self {
    Config {
      basic: None,
      esrep: None
    }
  }
}
