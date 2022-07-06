mod basic;
mod login;

use anyhow::Result;
use serde::{Serialize, Deserialize};
use crate::tools::get_save_path;
use basic::*;
use login::*;

#[derive(Serialize, Deserialize)]
pub struct Storage {
  pub basic: BasicStorage,
  pub login: LoginStorage,
}

impl Storage {
  pub async fn load(path: &str) -> Result<Self> {
    let file = get_save_path(path);
    let storage_str = tokio::fs::read_to_string(file).await?;
    let storage = serde_json::from_str(&storage_str)?;
    Ok(storage)
  }
  pub async fn save(&self, path: &str) -> Result<()> {
    let storage_str = serde_json::to_string(&self)?;
    let file = get_save_path(path);
    tokio::fs::write(file, storage_str).await?;
    Ok(())
  }
  pub fn new() -> Self {
    Storage {
      basic: BasicStorage {
        uid: None,
        name: None,
      },
      login: LoginStorage {
        cookie_tgc: None,
        cookie_jwt: None,
      },
    }
  }
}
